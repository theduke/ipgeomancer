use anyhow::Result;
use clap::{Parser, Subcommand};
use ipgeom_rir::Store;
use maxminddb::Reader;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;

/// Command line interface for ipgeomancer.
#[derive(Parser)]
#[command(name = "ipgeom", version, about = "Tools for IP geolocation")]
struct Cli {
    /// Directory where downloaded RIR data and other artifacts are stored
    #[arg(long, short, env = "IPGEOM_DATA_DIR", default_value = "data")]
    data_dir: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage the local RIR data store
    #[command(subcommand)]
    Store(StoreCmd),
    /// Query GeoIP database files
    #[command(subcommand)]
    Ipdb(IpdbCmd),
}

#[derive(Subcommand)]
enum StoreCmd {
    /// Download database dumps from all RIRs
    Update,
    /// Build a MaxMind GeoIP database from stored RIR data
    BuildGeoipdb {
        /// Path of the GeoIP database file to create
        path: PathBuf,
    },
}

#[derive(Subcommand)]
enum IpdbCmd {
    /// Lookup an IP address in a GeoIP database
    Lookup {
        /// IP address to query
        ip: IpAddr,
        /// Path to the GeoIP database file
        #[arg(short, long)]
        db: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("ipgeom=debug,info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Store(cmd) => handle_store(cli.data_dir, cmd).await?,
        Commands::Ipdb(cmd) => handle_ipdb(cmd)?,
    }

    Ok(())
}

async fn handle_store(data_dir: PathBuf, cmd: StoreCmd) -> Result<()> {
    let store = Store::new(data_dir);
    match cmd {
        StoreCmd::Update => store.update().await?,
        StoreCmd::BuildGeoipdb { path } => store.write_geoip_db(path)?,
    }
    Ok(())
}

fn handle_ipdb(cmd: IpdbCmd) -> Result<()> {
    match cmd {
        IpdbCmd::Lookup { ip, db } => {
            #[derive(Debug, Deserialize, Serialize)]
            struct Record {
                country: String,
            }

            let reader = Reader::open_readfile(db)?;
            if let Some(record) = reader.lookup::<Record>(ip)? {
                println!("{}", serde_json::to_string_pretty(&record)?);
            } else {
                eprintln!("address not found");
            }
        }
    }
    Ok(())
}
