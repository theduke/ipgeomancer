use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;
use ipgeom_rir::Store;

pub mod sqlite;
pub mod update;

#[derive(Subcommand)]
pub enum StoreCmd {
    /// Download database dumps from all RIRs
    Update(update::Update),
    /// Build a MaxMind GeoIP database from stored RIR data
    BuildGeoipdb {
        /// Path of the GeoIP database file to create
        path: PathBuf,
    },
    /// Populate a SQLite database from stored RIR data
    BuildSqlitedb(sqlite::SqliteDbCmd),
}

pub fn handle(data_dir: PathBuf, cmd: StoreCmd) -> Result<()> {
    let store = Store::new(data_dir)?;
    match cmd {
        StoreCmd::Update(args) => update::handle(&store, args)?,
        StoreCmd::BuildGeoipdb { path } => store.write_geoip_db(path)?,
        StoreCmd::BuildSqlitedb(args) => sqlite::handle(&store, args)?,
    }
    Ok(())
}
