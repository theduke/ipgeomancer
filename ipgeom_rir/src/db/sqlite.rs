use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;
use std::sync::{Arc, Mutex};

use super::{Database, object_key};
use ipgeom_rpsl::{ObjectType, RpslObject};
use rusqlite::{OptionalExtension, params};

/// Simple SQLite implementation of [`Database`].
#[derive(Debug, Clone)]
pub struct SqliteDb {
    conn: Arc<Mutex<rusqlite::Connection>>,
}

impl SqliteDb {
    /// Configure SQLite connection pragmas for better write performance.
    fn configure_connection(
        conn: &rusqlite::Connection,
        memory: bool,
    ) -> Result<(), anyhow::Error> {
        if memory {
            conn.pragma_update(None, "journal_mode", "MEMORY")?;
            conn.pragma_update(None, "synchronous", "OFF")?;
        } else {
            conn.pragma_update(None, "journal_mode", "WAL")?;
            conn.pragma_update(None, "synchronous", "NORMAL")?;
        }
        conn.pragma_update(None, "temp_store", "MEMORY")?;
        Ok(())
    }

    /// Open or create a database at the given path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let conn = rusqlite::Connection::open(path)?;
        Self::configure_connection(&conn, false)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Create an in-memory database (used for tests).
    #[cfg(test)]
    pub fn memory() -> Result<Self, anyhow::Error> {
        let conn = rusqlite::Connection::open_in_memory()?;
        Self::configure_connection(&conn, true)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    fn ensure_schema_table(&self) -> Result<(), anyhow::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY)",
            [],
        )?;
        Ok(())
    }

    fn current_version(&self) -> Result<i64, anyhow::Error> {
        self.ensure_schema_table()?;
        let conn = self.conn.lock().unwrap();
        Ok(conn.query_row(
            "SELECT COALESCE(MAX(version),0) FROM schema_migrations",
            [],
            |r| r.get(0),
        )?)
    }

    fn set_version(&self, version: i64) -> Result<(), anyhow::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO schema_migrations (version) VALUES (?)",
            [version],
        )?;
        Ok(())
    }

    fn upsert_rpsl_object_tx(
        &self,
        tx: &rusqlite::Transaction<'_>,
        obj: &RpslObject,
    ) -> Result<(), anyhow::Error> {
        let (obj_type, source, json) = match obj {
            RpslObject::Inetnum(i) => ("inetnum", i.source.clone(), serde_json::to_string(i)?),
            RpslObject::Inet6num(i) => ("inet6num", i.source.clone(), serde_json::to_string(i)?),
            RpslObject::AutNum(i) => ("aut-num", i.source.clone(), serde_json::to_string(i)?),
            RpslObject::Person(i) => ("person", i.source.clone(), serde_json::to_string(i)?),
            RpslObject::Role(i) => ("role", i.source.clone(), serde_json::to_string(i)?),
            RpslObject::Organisation(i) => {
                ("organisation", i.source.clone(), serde_json::to_string(i)?)
            }
            RpslObject::Mntner(i) => ("mntner", i.source.clone(), serde_json::to_string(i)?),
            RpslObject::Route(i) => ("route", i.source.clone(), serde_json::to_string(i)?),
            RpslObject::Route6(i) => ("route6", i.source.clone(), serde_json::to_string(i)?),
            RpslObject::Other(o) => (
                match o.obj_type() {
                    ObjectType::Other(name) => name.as_str(),
                    _ => "other",
                },
                None,
                serde_json::to_string(o)?,
            ),
        };
        let key = object_key(obj);
        tx
            .prepare_cached(
                "INSERT INTO rpsl (obj_type, obj_key, source, json) VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(obj_type, obj_key) DO UPDATE SET source=excluded.source, json=excluded.json",
            )?
            .execute(params![obj_type, key, source, json])?;

        let obj_id: i64 = tx
            .prepare_cached("SELECT id FROM rpsl WHERE obj_type=?1 AND obj_key=?2")?
            .query_row(params![obj_type, key], |r| r.get(0))?;

        tx.prepare_cached("DELETE FROM ipv4_geo WHERE obj_id=?1")?
            .execute([obj_id])?;
        tx.prepare_cached("DELETE FROM ipv6_geo WHERE obj_id=?1")?
            .execute([obj_id])?;

        match obj {
            RpslObject::Inetnum(inet) => {
                if let Some(country) = &inet.country {
                    for net in &inet.inetnum {
                        let start: u32 = net.network().into();
                        let end: u32 = net.broadcast().into();
                        tx
                            .prepare_cached(
                                "INSERT INTO ipv4_geo (start, end, country, obj_id) VALUES (?1, ?2, ?3, ?4)
                                 ON CONFLICT(start, end, obj_id) DO UPDATE SET country=excluded.country",
                            )?
                            .execute(params![start as i64, end as i64, country, obj_id])?;
                    }
                }
            }
            RpslObject::Inet6num(inet) => {
                if let Some(country) = &inet.country {
                    for net in &inet.inet6num {
                        let start: u128 = net.network().into();
                        let end: u128 = net.broadcast().into();
                        let sb = start.to_be_bytes();
                        let eb = end.to_be_bytes();
                        tx
                            .prepare_cached(
                                "INSERT INTO ipv6_geo (start, end, country, obj_id) VALUES (?1, ?2, ?3, ?4)
                                 ON CONFLICT(start, end, obj_id) DO UPDATE SET country=excluded.country",
                            )?
                            .execute(params![sb.as_slice(), eb.as_slice(), country, obj_id])?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Database for SqliteDb {
    fn migrate(&self) -> Result<(), anyhow::Error> {
        let ver = self.current_version()?;
        if ver < 1 {
            let conn = self.conn.lock().unwrap();
            conn.execute_batch(
                r#"
                CREATE TABLE rpsl (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    obj_type TEXT NOT NULL,
                    obj_key TEXT NOT NULL,
                    source TEXT,
                    json TEXT NOT NULL,
                    UNIQUE(obj_type, obj_key)
                );
                CREATE TABLE ipv4_geo (
                    start INTEGER NOT NULL,
                    end INTEGER NOT NULL,
                    country TEXT NOT NULL,
                    obj_id INTEGER NOT NULL REFERENCES rpsl(id) ON DELETE CASCADE,
                    UNIQUE(start, end, obj_id)
                );
                CREATE INDEX ipv4_geo_idx ON ipv4_geo(start, end);
                CREATE TABLE ipv6_geo (
                    start BLOB NOT NULL,
                    end BLOB NOT NULL,
                    country TEXT NOT NULL,
                    obj_id INTEGER NOT NULL REFERENCES rpsl(id) ON DELETE CASCADE,
                    UNIQUE(start, end, obj_id)
                );
                CREATE INDEX ipv6_geo_idx ON ipv6_geo(start, end);
                "#,
            )?;
            drop(conn);
            self.set_version(1)?;
        }
        Ok(())
    }

    fn upsert_rpsl_object(&self, obj: &RpslObject) -> Result<(), anyhow::Error> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        self.upsert_rpsl_object_tx(&tx, obj)?;
        tx.commit()?;
        Ok(())
    }

    fn upsert_rpsl_objects(&self, objs: &[RpslObject]) -> Result<(), anyhow::Error> {
        if objs.is_empty() {
            return Ok(());
        }
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        for obj in objs {
            self.upsert_rpsl_object_tx(&tx, obj)?;
        }
        tx.commit()?;
        Ok(())
    }

    fn lookup_ipv4(&self, addr: Ipv4Addr) -> Result<Option<String>, anyhow::Error> {
        let num: u32 = addr.into();
        let conn = self.conn.lock().unwrap();
        let res = conn
            .query_row(
                "SELECT country FROM ipv4_geo WHERE start <= ?1 AND end >= ?1 LIMIT 1",
                [num as i64],
                |r| r.get(0),
            )
            .optional()?;
        Ok(res)
    }

    fn lookup_ipv4_with_obj(
        &self,
        addr: Ipv4Addr,
    ) -> Result<Option<(String, String, String)>, anyhow::Error> {
        let num: u32 = addr.into();
        let conn = self.conn.lock().unwrap();
        let res = conn
            .query_row(
                "SELECT country, obj_type, obj_key FROM ipv4_geo \
                 JOIN rpsl ON ipv4_geo.obj_id = rpsl.id \
                 WHERE start <= ?1 AND end >= ?1 LIMIT 1",
                [num as i64],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()?;
        Ok(res)
    }

    fn lookup_ipv6(&self, addr: Ipv6Addr) -> Result<Option<String>, anyhow::Error> {
        let bytes = addr.octets();
        let conn = self.conn.lock().unwrap();
        let res = conn
            .query_row(
                "SELECT country FROM ipv6_geo WHERE start <= ?1 AND end >= ?1 ORDER BY (end-start) ASC LIMIT 1",
                [bytes.as_slice()],
                |r| r.get(0),
            )
            .optional()?;
        Ok(res)
    }

    fn lookup_ipv6_with_obj(
        &self,
        addr: Ipv6Addr,
    ) -> Result<Option<(String, String, String)>, anyhow::Error> {
        let bytes = addr.octets();
        let conn = self.conn.lock().unwrap();
        let res = conn
            .query_row(
                "SELECT country, obj_type, obj_key FROM ipv6_geo \
                 JOIN rpsl ON ipv6_geo.obj_id = rpsl.id \
                 WHERE start <= ?1 AND end >= ?1 ORDER BY (end-start) ASC LIMIT 1",
                [bytes.as_slice()],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()?;
        Ok(res)
    }

    fn lookup_ipv4_all(&self, addr: Ipv4Addr) -> Result<Vec<String>, anyhow::Error> {
        let num: u32 = addr.into();
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT country FROM ipv4_geo WHERE start <= ?1 AND end >= ?1 ORDER BY (end-start) ASC",
        )?;
        let rows = stmt
            .query_map([num as i64], |r| r.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(rows)
    }

    fn lookup_ipv6_all(&self, addr: Ipv6Addr) -> Result<Vec<String>, anyhow::Error> {
        let bytes = addr.octets();
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT country FROM ipv6_geo WHERE start <= ?1 AND end >= ?1 ORDER BY (end-start) ASC",
        )?;
        let rows = stmt
            .query_map([bytes.as_slice()], |r| r.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(rows)
    }

    fn get_object(&self, obj_type: &str, obj_key: &str) -> Result<Option<String>, anyhow::Error> {
        let conn = self.conn.lock().unwrap();
        let res = conn
            .query_row(
                "SELECT json FROM rpsl WHERE obj_type = ?1 AND obj_key = ?2",
                params![obj_type, obj_key],
                |r| r.get(0),
            )
            .optional()?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ipgeom_rpsl::{RpslObject, parse_objects};

    #[test]
    fn lookup_ipv4_all_overlaps() {
        let db = SqliteDb::memory().unwrap();
        db.migrate().unwrap();

        let obj1 = RpslObject::try_from(
            parse_objects("inetnum: 192.0.2.0/25\ncountry: AA\nsource: TEST\n")
                .unwrap()
                .remove(0),
        )
        .unwrap();
        let obj2 = RpslObject::try_from(
            parse_objects("inetnum: 192.0.2.0/24\ncountry: BB\nsource: TEST\n")
                .unwrap()
                .remove(0),
        )
        .unwrap();

        db.upsert_rpsl_object(&obj1).unwrap();
        db.upsert_rpsl_object(&obj2).unwrap();

        let res = db.lookup_ipv4_all("192.0.2.1".parse().unwrap()).unwrap();
        assert_eq!(res, vec!["AA".to_string(), "BB".to_string()]);
    }
}
