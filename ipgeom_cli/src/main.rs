use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cmd;

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
    Store(cmd::store::StoreCmd),
    /// Query GeoIP database files
    #[command(subcommand)]
    Ipdb(cmd::ipdb::IpdbCmd),
    /// Work with RPSL files
    #[command(subcommand)]
    Rpsl(cmd::rpsl::RpslCmd),
}

fn main() -> Result<()> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("ipgeom=debug,info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Store(cmd) => cmd::store::handle(cli.data_dir, cmd)?,
        Commands::Ipdb(cmd) => cmd::ipdb::handle(cmd)?,
        Commands::Rpsl(cmd) => cmd::rpsl::handle(cmd)?,
    }

    Ok(())
}
