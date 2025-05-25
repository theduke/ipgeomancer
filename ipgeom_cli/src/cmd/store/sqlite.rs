use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use ipgeom_rir::{SqliteDb, Store};

/// Populate a SQLite database using the contents of the store.
#[derive(Args)]
pub struct SqliteDbCmd {
    /// Path of the SQLite database file to create
    pub path: PathBuf,
}

pub fn handle(store: &Store, args: SqliteDbCmd) -> Result<()> {
    let db = SqliteDb::open(&args.path)?;
    store.persist_to_db(&db, Default::default())
}
