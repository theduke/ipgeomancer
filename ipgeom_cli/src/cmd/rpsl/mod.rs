use anyhow::Result;
use clap::Subcommand;

pub mod print;

#[derive(Subcommand)]
pub enum RpslCmd {
    /// Print RPSL objects from a file
    Print(print::Print),
}

pub fn handle(cmd: RpslCmd) -> Result<()> {
    match cmd {
        RpslCmd::Print(args) => print::handle(args),
    }
}
