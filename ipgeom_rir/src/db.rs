use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;

use rusqlite::{OptionalExtension, params};

use ipgeom_rpsl::RpslObject;

/// Trait describing database backends that can store RPSL information and
/// provide IP geolocation lookups.
pub trait Database: Send {
    /// Run pending migrations if necessary.
    fn migrate(&self) -> Result<(), anyhow::Error>;

    /// Insert a single RPSL object as JSON into the appropriate table.
    fn insert_rpsl_object(&self, obj: &RpslObject) -> Result<(), anyhow::Error>;

    /// Insert an IPv4 range with an associated country code.
    fn insert_ipv4_geo(&self, start: u32, end: u32, country: &str) -> Result<(), anyhow::Error>;

    /// Insert an IPv6 range with an associated country code.
    fn insert_ipv6_geo(&self, start: u128, end: u128, country: &str) -> Result<(), anyhow::Error>;

    /// Perform a lookup for an IPv4 address. Returns the country code if found.
    fn lookup_ipv4(&self, addr: Ipv4Addr) -> Result<Option<String>, anyhow::Error>;

    /// Perform a lookup for an IPv6 address. Returns the country code if found.
    fn lookup_ipv6(&self, addr: Ipv6Addr) -> Result<Option<String>, anyhow::Error>;
}

/// Simple SQLite implementation of [`Database`].
#[derive(Debug)]
pub struct SqliteDb {
    conn: rusqlite::Connection,
}

impl SqliteDb {
    /// Open or create a database at the given path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let conn = rusqlite::Connection::open(path)?;
        Ok(Self { conn })
    }

    /// Create an in-memory database (used for tests).
    #[cfg(test)]
    pub fn memory() -> Result<Self, anyhow::Error> {
        let conn = rusqlite::Connection::open_in_memory()?;
        Ok(Self { conn })
    }

    fn ensure_schema_table(&self) -> Result<(), anyhow::Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY)",
            [],
        )?;
        Ok(())
    }

    fn current_version(&self) -> Result<i64, anyhow::Error> {
        self.ensure_schema_table()?;
        Ok(self.conn.query_row(
            "SELECT COALESCE(MAX(version),0) FROM schema_migrations",
            [],
            |r| r.get(0),
        )?)
    }

    fn set_version(&self, version: i64) -> Result<(), anyhow::Error> {
        self.conn.execute(
            "INSERT INTO schema_migrations (version) VALUES (?)",
            [version],
        )?;
        Ok(())
    }
}

impl Database for SqliteDb {
    fn migrate(&self) -> Result<(), anyhow::Error> {
        let ver = self.current_version()?;
        if ver < 1 {
            self.conn.execute_batch(
                r#"
                CREATE TABLE inetnum (id INTEGER PRIMARY KEY, json TEXT NOT NULL);
                CREATE TABLE inet6num (id INTEGER PRIMARY KEY, json TEXT NOT NULL);
                CREATE TABLE autnum (id INTEGER PRIMARY KEY, json TEXT NOT NULL);
                CREATE TABLE person (id INTEGER PRIMARY KEY, json TEXT NOT NULL);
                CREATE TABLE role (id INTEGER PRIMARY KEY, json TEXT NOT NULL);
                CREATE TABLE organisation (id INTEGER PRIMARY KEY, json TEXT NOT NULL);
                CREATE TABLE mntner (id INTEGER PRIMARY KEY, json TEXT NOT NULL);
                CREATE TABLE route (id INTEGER PRIMARY KEY, json TEXT NOT NULL);
                CREATE TABLE route6 (id INTEGER PRIMARY KEY, json TEXT NOT NULL);
                CREATE TABLE ipv4_geo (start INTEGER NOT NULL, end INTEGER NOT NULL, country TEXT NOT NULL);
                CREATE INDEX ipv4_geo_idx ON ipv4_geo(start, end);
                CREATE TABLE ipv6_geo (start BLOB NOT NULL, end BLOB NOT NULL, country TEXT NOT NULL);
                CREATE INDEX ipv6_geo_idx ON ipv6_geo(start, end);
                "#,
            )?;
            self.set_version(1)?;
        }
        Ok(())
    }

    fn insert_rpsl_object(&self, obj: &RpslObject) -> Result<(), anyhow::Error> {
        let (table, json) = match obj {
            RpslObject::Inetnum(i) => ("inetnum", serde_json::to_string(i)?),
            RpslObject::Inet6num(i) => ("inet6num", serde_json::to_string(i)?),
            RpslObject::AutNum(i) => ("autnum", serde_json::to_string(i)?),
            RpslObject::Person(i) => ("person", serde_json::to_string(i)?),
            RpslObject::Role(i) => ("role", serde_json::to_string(i)?),
            RpslObject::Organisation(i) => ("organisation", serde_json::to_string(i)?),
            RpslObject::Mntner(i) => ("mntner", serde_json::to_string(i)?),
            RpslObject::Route(i) => ("route", serde_json::to_string(i)?),
            RpslObject::Route6(i) => ("route6", serde_json::to_string(i)?),
            RpslObject::Other(o) => ("route", serde_json::to_string(o)?),
        };
        let sql = format!("INSERT INTO {} (json) VALUES (?1)", table);
        self.conn.execute(sql.as_str(), [json])?;
        Ok(())
    }

    fn insert_ipv4_geo(&self, start: u32, end: u32, country: &str) -> Result<(), anyhow::Error> {
        self.conn.execute(
            "INSERT INTO ipv4_geo (start, end, country) VALUES (?1, ?2, ?3)",
            params![start as i64, end as i64, country],
        )?;
        Ok(())
    }

    fn insert_ipv6_geo(&self, start: u128, end: u128, country: &str) -> Result<(), anyhow::Error> {
        let start_bytes = start.to_be_bytes();
        let end_bytes = end.to_be_bytes();
        self.conn.execute(
            "INSERT INTO ipv6_geo (start, end, country) VALUES (?1, ?2, ?3)",
            params![start_bytes.as_slice(), end_bytes.as_slice(), country],
        )?;
        Ok(())
    }

    fn lookup_ipv4(&self, addr: Ipv4Addr) -> Result<Option<String>, anyhow::Error> {
        let num: u32 = addr.into();
        let res = self
            .conn
            .query_row(
                "SELECT country FROM ipv4_geo WHERE start <= ?1 AND end >= ?1 LIMIT 1",
                [num as i64],
                |r| r.get(0),
            )
            .optional()?;
        Ok(res)
    }

    fn lookup_ipv6(&self, addr: Ipv6Addr) -> Result<Option<String>, anyhow::Error> {
        let bytes = addr.octets();
        let res = self
            .conn
            .query_row(
                "SELECT country FROM ipv6_geo WHERE start <= ?1 AND end >= ?1 LIMIT 1",
                [bytes.as_slice()],
                |r| r.get(0),
            )
            .optional()?;
        Ok(res)
    }
}
