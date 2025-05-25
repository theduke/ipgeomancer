use anyhow::Result;
use clap::Subcommand;

pub mod lookup;

#[derive(Subcommand)]
pub enum IpdbCmd {
    /// Lookup an IP address in a GeoIP database
    Lookup(lookup::Lookup),
}

pub fn handle(cmd: IpdbCmd) -> Result<()> {
    match cmd {
        IpdbCmd::Lookup(args) => lookup::handle(args),
    }
}
