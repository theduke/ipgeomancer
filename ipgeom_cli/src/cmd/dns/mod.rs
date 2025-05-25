use anyhow::Result;
use clap::Subcommand;

pub mod query;

#[derive(Subcommand)]
pub enum DnsCmd {
    /// Query DNS records
    Query(query::QueryCmd),
}

pub async fn handle(cmd: DnsCmd) -> Result<()> {
    match cmd {
        DnsCmd::Query(q) => query::handle(q).await,
    }
}
