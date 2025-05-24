use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use flate2::read::GzDecoder;
use ipgeom_rpsl::{RpslObject, parse_objects_read_iter};

use crate::{Client, DbData, Rir, registry, types};

/// Persistent store for RIR database dumps.
#[derive(Debug)]
pub struct Store {
    data_dir: PathBuf,
    client: Client,
    rirs: HashMap<types::Rir, Box<dyn Rir>>,
}

impl Store {
    /// Create a new store using default RIR implementations.
    pub fn new<P: Into<PathBuf>>(data_dir: P) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("ipgeomancer")
            .build()
            .expect("failed to build client");

        let rirs: HashMap<types::Rir, Box<dyn Rir>> = [
            (
                types::Rir::Arin,
                Box::new(registry::arin::Arin {}) as Box<dyn Rir>,
            ),
            (types::Rir::Apnic, Box::new(registry::apnic::Apnic {})),
            (types::Rir::Ripe, Box::new(registry::ripe::Ripe {})),
            (types::Rir::Lacnic, Box::new(registry::lacnic::Lacnic {})),
            (types::Rir::Afrinic, Box::new(registry::afrinic::Afrinic {})),
        ]
        .into_iter()
        .collect();

        Self {
            data_dir: data_dir.into(),
            client,
            rirs,
        }
    }

    /// Create a store with custom RIR implementations (useful for testing).
    pub fn with_rirs<P: Into<PathBuf>>(
        data_dir: P,
        rirs: HashMap<types::Rir, Box<dyn Rir>>,
    ) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("ipgeomancer")
            .build()
            .expect("failed to build client");
        Self {
            data_dir: data_dir.into(),
            client,
            rirs,
        }
    }

    /// Download the databases from all configured RIRs.
    pub async fn update(&self) -> Result<(), anyhow::Error> {
        for (rir, handler) in &self.rirs {
            let data = handler.download_rpsl_db(&self.client).await?;
            self.store_data(*rir, data)?;
        }
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
    ) -> Result<impl Iterator<Item = Result<RpslObject, ipgeom_rpsl::ParseError>>, anyhow::Error>
    {
        let file = File::open(self.db_path(rir))?;
        let reader = BufReader::new(file);
        Ok(parse_objects_read_iter(reader).map(|res| {
            res.map(|obj| match RpslObject::try_from(obj.clone()) {
                Ok(t) => t,
                Err(_) => ipgeom_rpsl::RpslObject::Other(obj),
            })
        }))
    }

    /// Iterate over typed RPSL objects from all stored registries.
    pub fn all_objects_iter(
        &self,
    ) -> Result<impl Iterator<Item = Result<RpslObject, ipgeom_rpsl::ParseError>>, anyhow::Error>
    {
        let mut iters = Vec::new();
        for rir in types::Rir::ALL.iter() {
            if self.rirs.contains_key(rir) {
                iters.push(self.objects_iter(*rir)?);
            }
        }
        Ok(iters.into_iter().flatten())
    }
}

#[cfg(test)]
mod tests {
    use crate::RirKind;

    use super::*;

    use std::collections::HashMap;

    use std::fs;
    use std::pin::Pin;
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

    impl Rir for MockRir {
        fn download_rpsl_db<'a>(
            &'a self,
            _client: &'a Client,
        ) -> Pin<Box<dyn Future<Output = Result<DbData, anyhow::Error>> + Send + 'a>> {
            let data = self.data.clone();
            Box::pin(async move {
                Ok(DbData {
                    gzip: false,
                    reader: Box::new(std::io::Cursor::new(data)),
                })
            })
        }
    }

    pub fn mock_rir_data() -> String {
        "inetnum: 192.0.2.0/24\nnetname: TEST-NET\nsource: TST\n\n".to_string()
    }

    #[tokio::test]
    async fn store_download_and_iter() {
        let mut base = std::env::temp_dir();
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        base.push(format!("ipgeomancer_test_{}", t));
        fs::create_dir_all(&base).unwrap();
        let mut rirs: HashMap<RirKind, Box<dyn crate::Rir>> = HashMap::new();
        for rir in RirKind::ALL.iter() {
            rirs.insert(*rir, Box::new(MockRir::new(&mock_rir_data())));
        }

        let store = Store::with_rirs(&base, rirs);
        store.update().await.unwrap();

        for rir in RirKind::ALL.iter() {
            let mut iter = store.objects_iter(*rir).unwrap();
            let obj = iter.next().unwrap().unwrap();
            if let ipgeom_rpsl::RpslObject::Inetnum(inet) = obj {
                assert_eq!(inet.netname.as_deref(), Some("TEST-NET"));
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
        let count = all.count();
        assert_eq!(count, RirKind::ALL.len() - 1);
    }
}
