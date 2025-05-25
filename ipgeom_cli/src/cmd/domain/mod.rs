use anyhow::Result;
use clap::Subcommand;

pub mod check_certificate;

#[derive(Subcommand)]
pub enum DomainCmd {
    /// Check TLS certificate for a domain
    CheckCertificate(check_certificate::CheckCertCmd),
}

pub async fn handle(cmd: DomainCmd) -> Result<()> {
    match cmd {
        DomainCmd::CheckCertificate(c) => check_certificate::handle(c).await,
    }
}
