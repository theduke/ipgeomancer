use anyhow::Result;
use clap::Args;
use maxminddb::Reader;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Args)]
pub struct Lookup {
    /// IP address to query
    pub ip: IpAddr,
    /// Path to the GeoIP database file
    #[arg(short, long)]
    pub db: PathBuf,
}

pub fn handle(args: Lookup) -> Result<()> {
    #[derive(Debug, Deserialize, Serialize)]
    struct Record {
        country: String,
    }

    let reader = Reader::open_readfile(args.db)?;
    if let Some(record) = reader.lookup::<Record>(args.ip)? {
        println!("{}", serde_json::to_string_pretty(&record)?);
    } else {
        eprintln!("address not found");
    }
    Ok(())
}
