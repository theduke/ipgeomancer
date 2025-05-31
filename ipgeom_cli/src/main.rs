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
    /// Domain related commands
    #[command(subcommand)]
    Domain(cmd::domain::DomainCmd),
    /// Run the HTTP server
    Server(cmd::server::ServerCmd),
    /// Perform DNS queries
    #[command(subcommand)]
    Dns(cmd::dns::DnsCmd),
    /// Domain WHOIS lookup
    Whois(cmd::whois::WhoisCmd),
    /// Generic RDAP query
    Rdap(cmd::rdap::RdapCmd),
    /// Ping a host
    Ping(cmd::ping::PingCmd),
    /// Trace the route to a host
    Traceroute(cmd::traceroute::TracerouteCmd),
    /// Generate password hashes
    MakePasswordHash(cmd::password_hash::MakePasswordHashCmd),
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        tracing_subscriber::EnvFilter::new("ipgeom=debug,tower_http::trace=debug,info")
    });
    eprintln!("Using log filter: {}", filter);
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Store(cmd) => cmd::store::handle(cli.data_dir, cmd)?,
        Commands::Ipdb(cmd) => cmd::ipdb::handle(cmd)?,
        Commands::Rpsl(cmd) => cmd::rpsl::handle(cmd)?,
        Commands::Domain(cmd) => cmd::domain::handle(cmd).await?,
        Commands::Server(cmd) => cmd::server::handle(cmd).await?,
        Commands::Dns(cmd) => cmd::dns::handle(cmd).await?,
        Commands::Whois(cmd) => cmd::whois::handle(cmd).await?,
        Commands::Rdap(cmd) => cmd::rdap::handle(cmd).await?,
        Commands::Ping(cmd) => cmd::ping::handle(cmd).await?,
        Commands::Traceroute(cmd) => cmd::traceroute::handle(cmd).await?,
        Commands::MakePasswordHash(cmd) => cmd::password_hash::handle(cmd)?,
    }

    Ok(())
}
