use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use flate2::read::GzDecoder;
use ipgeom_rpsl::{parse_objects_read_iter, RpslObject};

use crate::db::Database;

use crate::{registry, types, Client, DbData, RirProvider};

/// Persistent store for RIR database dumps.
#[derive(Debug)]
pub struct Store {
    data_dir: PathBuf,
    client: Client,
    rirs: HashMap<types::Rir, Box<dyn RirProvider>>,
}

/// Options controlling what data is persisted into a database.
#[derive(Debug, Clone, Copy)]
pub struct PersistFilter {
    /// Persist all RPSL objects into dedicated tables.
    pub rpsl_objects: bool,
    pub rpsl_inetnum: bool,
}

impl Default for PersistFilter {
    fn default() -> Self {
        Self {
            rpsl_objects: false,
            rpsl_inetnum: true,
        }
    }
}

impl Store {
    /// Create a new store using default RIR implementations.
    pub fn new<P: Into<PathBuf>>(data_dir: P) -> Result<Self, anyhow::Error> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("ipgeomancer")
            .build()?;

        let rirs: HashMap<types::Rir, Box<dyn RirProvider>> = [
            // (
            //     types::Rir::Arin,
            //     Box::new(registry::arin::Arin {}) as Box<dyn RirProvider>,
            // ),
            // (types::Rir::Apnic, Box::new(registry::apnic::Apnic {})),
            (
                types::Rir::Ripe,
                Box::new(registry::ripe::Ripe {}) as Box<dyn RirProvider>,
            ),
            // (types::Rir::Lacnic, Box::new(registry::lacnic::Lacnic {})),
            (
                types::Rir::Afrinic,
                Box::new(registry::afrinic::Afrinic {}) as Box<dyn RirProvider>,
            ),
        ]
        .into_iter()
        .collect();

        Ok(Self {
            data_dir: data_dir.into(),
            client,
            rirs,
        })
    }

    /// Create a store with custom RIR implementations (useful for testing).
    pub fn with_rirs<P: Into<PathBuf>>(
        data_dir: P,
        rirs: HashMap<types::Rir, Box<dyn RirProvider>>,
    ) -> Result<Self, anyhow::Error> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("ipgeomancer")
            .build()?;
        Ok(Self {
            data_dir: data_dir.into(),
            client,
            rirs,
        })
    }

    /// Download the databases from all configured RIRs.
    pub fn update(&self) -> Result<(), anyhow::Error> {
        tracing::info!("Updating RIR databases in {}", self.data_dir.display());

        for (rir, handler) in &self.rirs {
            tracing::debug!("Downloading RPSL data for {}", rir.name());
            let data = handler.download_rpsl_db(&self.client)?;
            self.store_data(*rir, data)?;
            tracing::info!("Updated RPSL db for {}", rir.name());
        }
        tracing::info!("RIR databases updated successfully");
        Ok(())
    }

    fn db_path(&self, rir: types::Rir) -> PathBuf {
        self.data_dir
            .join("rir")
            .join(rir.name())
            .join("db")
            .join("latest.rpsl")
    }

    fn store_data(&self, rir: types::Rir, mut data: DbData) -> Result<(), anyhow::Error> {
        let file_path = self.db_path(rir);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = File::create(file_path)?;
        if data.gzip {
            let mut reader = GzDecoder::new(data.reader);
            std::io::copy(&mut reader, &mut file)?;
        } else {
            std::io::copy(&mut data.reader, &mut file)?;
        }
        Ok(())
    }

    /// Iterate over typed RPSL objects stored for a given registry.
    pub fn objects_iter(
        &self,
        rir: types::Rir,
    ) -> Result<impl Iterator<Item = Result<RpslObject, anyhow::Error>>, anyhow::Error> {
        let file = File::open(self.db_path(rir))?;
        let reader = BufReader::new(file);
        let iter = parse_objects_read_iter(reader).map(|res| {
            let obj = res?;
            RpslObject::try_from(obj)
        });
        Ok(iter)
    }

    /// Iterate over typed RPSL objects from all stored registries.
    pub fn all_objects_iter(
        &self,
    ) -> Result<impl Iterator<Item = Result<RpslObject, anyhow::Error>>, anyhow::Error> {
        let mut iters = Vec::new();
        for rir in types::Rir::ALL.iter() {
            if self.rirs.contains_key(rir) {
                iters.push(self.objects_iter(*rir)?);
            }
        }
        Ok(iters.into_iter().flatten())
    }
    /// Persist stored objects into a database using the provided filter.
    pub fn persist_to_db<D: Database>(
        &self,
        db: &D,
        filter: PersistFilter,
    ) -> Result<(), anyhow::Error> {
        const BATCH_SIZE: usize = 1000;

        db.migrate()?;
        tracing::info!("Persisting store into database");

        let mut batch = Vec::with_capacity(BATCH_SIZE);
        let mut count = 0usize;

        for obj_res in self.all_objects_iter()? {
            let obj = obj_res?;

            if filter.rpsl_objects {
                batch.push(obj);
                count += 1;
            } else if filter.rpsl_inetnum && (obj.is_inetnum() || obj.is_inet6num()) {
                // Only store inetnum/inet6num objects if the filter allows it
                batch.push(obj);
                count += 1;
            } else {
                // Skip other object types
                continue;
            }

            if batch.len() >= BATCH_SIZE {
                tracing::debug!(count = batch.len(), "insert rpsl batch");
                db.upsert_rpsl_objects(&batch)?;
                batch.clear();
            }
        }

        if !batch.is_empty() {
            tracing::debug!(count = batch.len(), "insert final rpsl batch");
            db.upsert_rpsl_objects(&batch)?;
        }

        tracing::info!(rpsl_objects = count, "persisted store successfully");

        Ok(())
    }

    /// Build a GeoIP2 database from all stored objects.
    pub fn write_geoip_db<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), anyhow::Error> {
        use maxminddb_writer::{
            metadata::{IpVersion, Metadata},
            paths::IpAddrWithMask,
            Database,
        };
        use serde::Serialize;
        use std::time::{SystemTime, UNIX_EPOCH};

        #[derive(Serialize)]
        struct Record {
            country: String,
        }

        let path = path.as_ref();

        let mut metadata = Metadata::default();
        metadata.ip_version = IpVersion::V6;
        metadata.database_type = "GeoIP2-Country".into();
        metadata.languages = vec!["en".into()];
        metadata.binary_format_major_version = 2;
        metadata.binary_format_minor_version = 0;
        metadata.build_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        metadata.description = [(
            "en".to_string(),
            "ipgeomancer generated geoip database".to_string(),
        )]
        .into_iter()
        .collect();

        let mut db = Database::default();
        db.metadata = metadata;

        tracing::info!("Building GeoIP database to {}", path.display());

        for obj_res in self.all_objects_iter()? {
            let obj = obj_res.map_err(|e| anyhow::anyhow!(format!("{:?}", e)))?;
            match obj {
                RpslObject::Inetnum(inet) => {
                    if let Some(country) = &inet.country {
                        let mut nets = 0;
                        for net in &inet.inetnum {
                            let path = IpAddrWithMask::new(
                                std::net::IpAddr::V4(net.network()),
                                net.prefix_len(),
                            );
                            let data = db.insert_value(Record {
                                country: country.clone(),
                            })?;
                            tracing::debug!(?path, ?country, "adding inetnum object");
                            db.insert_node(path, data);
                            nets += 1;
                        }
                        if nets == 0 {
                            tracing::warn!(?inet, "inetnum object with no networks");
                        } else {
                            tracing::debug!(?inet, nets, "added inetnum object with networks");
                        }
                    } else {
                        tracing::warn!(?inet, "inetnum object without country");
                    }
                }
                RpslObject::Inet6num(inet) => {
                    if let Some(country) = inet.country {
                        for net in &inet.inet6num {
                            let path = IpAddrWithMask::new(
                                std::net::IpAddr::V6(net.network()),
                                net.prefix_len(),
                            );
                            let data = db.insert_value(Record {
                                country: country.clone(),
                            })?;
                            tracing::debug!(?path, ?country, "adding inet6num object");
                            db.insert_node(path, data);
                        }
                    } else {
                        tracing::warn!(?inet, "inet6num object without country");
                    }
                }
                _ => {}
            }
        }

        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        db.write_to(writer)?;

        tracing::info!(path=%path.display(), "GeoIP database written successfully");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::RirKind;
    use crate::SqliteDb;

    use super::*;

    use std::collections::HashMap;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Simple mock RIR used for tests.
    #[derive(Debug, Clone)]
    pub struct MockRir {
        data: String,
    }

    impl MockRir {
        pub fn new(data: &str) -> Self {
            Self {
                data: data.to_string(),
            }
        }
    }

    impl RirProvider for MockRir {
        fn build_rpsl_db_request(&self, _client: &Client) -> reqwest::blocking::RequestBuilder {
            unimplemented!("mock")
        }

        fn download_rpsl_db(&self, _client: &Client) -> Result<DbData, anyhow::Error> {
            Ok(DbData {
                gzip: false,
                reader: Box::new(std::io::Cursor::new(self.data.clone())),
            })
        }
    }

    pub fn mock_rir_data() -> String {
        "inetnum: 192.0.2.0/24\nnetname: TEST-NET\ncountry: ZZ\nsource: TST\n\n\
inet6num: 2001:db8::/32\nnetname: V6-NET\ncountry: ZZ\nsource: TST\n\n"
            .to_string()
    }

    #[test]
    fn store_download_and_iter() {
        let mut base = std::env::temp_dir();
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        base.push(format!("ipgeomancer_test_{}", t));
        fs::create_dir_all(&base).unwrap();
        let mut rirs: HashMap<RirKind, Box<dyn crate::RirProvider>> = HashMap::new();
        for rir in RirKind::ALL.iter() {
            rirs.insert(*rir, Box::new(MockRir::new(&mock_rir_data())));
        }

        let store = Store::with_rirs(&base, rirs).unwrap();
        store.update().unwrap();

        for rir in RirKind::ALL.iter() {
            let mut iter = store.objects_iter(*rir).unwrap();
            let obj = iter.next().unwrap().unwrap();
            if let ipgeom_rpsl::RpslObject::Inetnum(inet) = obj {
                assert_eq!(inet.netname.as_deref(), Some("TEST-NET"));
            } else {
                panic!("unexpected object");
            }
            let obj = iter.next().unwrap().unwrap();
            if let ipgeom_rpsl::RpslObject::Inet6num(inet) = obj {
                assert_eq!(inet.netname.as_deref(), Some("V6-NET"));
            } else {
                panic!("unexpected object");
            }
            assert!(iter.next().is_none());
        }

        let mut all = store.all_objects_iter().unwrap();
        let obj = all.next().unwrap().unwrap();
        if let ipgeom_rpsl::RpslObject::Inetnum(inet) = obj {
            assert_eq!(inet.netname.as_deref(), Some("TEST-NET"));
        } else {
            panic!("unexpected object");
        }
        let obj = all.next().unwrap().unwrap();
        if let ipgeom_rpsl::RpslObject::Inet6num(inet) = obj {
            assert_eq!(inet.netname.as_deref(), Some("V6-NET"));
        } else {
            panic!("unexpected object");
        }
        let count = all.count();
        assert_eq!(count, RirKind::ALL.len() * 2 - 2);
    }

    #[test]
    fn generate_geoip_db_file() {
        let mut base = std::env::temp_dir();
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        base.push(format!("ipgeomancer_test_db_{}", t));
        fs::create_dir_all(&base).unwrap();
        let mut rirs: HashMap<RirKind, Box<dyn crate::RirProvider>> = HashMap::new();
        for rir in RirKind::ALL.iter() {
            rirs.insert(*rir, Box::new(MockRir::new(&mock_rir_data())));
        }

        let store = Store::with_rirs(&base, rirs).unwrap();
        store.update().unwrap();

        let db_path = base.join("geoip.mmdb");
        store.write_geoip_db(&db_path).unwrap();

        let meta = fs::metadata(&db_path).unwrap();
        assert!(meta.len() > 0);
    }

    #[test]
    fn persist_to_sqlite_db() {
        let mut base = std::env::temp_dir();
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        base.push(format!("ipgeomancer_test_persist_{}", t));
        fs::create_dir_all(&base).unwrap();
        let mut rirs: HashMap<RirKind, Box<dyn crate::RirProvider>> = HashMap::new();
        for rir in RirKind::ALL.iter() {
            rirs.insert(*rir, Box::new(MockRir::new(&mock_rir_data())));
        }

        let store = Store::with_rirs(&base, rirs).unwrap();
        store.update().unwrap();

        let db = SqliteDb::memory().unwrap();
        store.persist_to_db(&db, PersistFilter::default()).unwrap();

        let (c, obj_type, key) = db
            .lookup_ipv4_with_obj("192.0.2.1".parse().unwrap())
            .unwrap()
            .unwrap();
        assert_eq!(c, "ZZ");
        let json = db.get_object(&obj_type, &key).unwrap().unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["netname"], "TEST-NET");

        let (c, obj_type, key) = db
            .lookup_ipv6_with_obj("2001:db8::1".parse().unwrap())
            .unwrap()
            .unwrap();
        assert_eq!(c, "ZZ");
        let json = db.get_object(&obj_type, &key).unwrap().unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["netname"], "V6-NET");
    }
}
