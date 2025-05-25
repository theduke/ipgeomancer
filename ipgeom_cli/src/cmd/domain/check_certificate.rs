use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct CheckCertCmd {
    /// Domain name to connect to
    pub domain: String,
}

pub async fn handle(args: CheckCertCmd) -> Result<()> {
    let info = ipgeom_query::fetch_certificate(&args.domain).await?;
    println!("Domain: {}", args.domain);
    println!("Subject: {}", info.subject);
    println!("Issuer: {}", info.issuer);
    println!("Valid From: {}", info.not_before);
    println!("Valid To: {}", info.not_after);
    println!("Validation: {}", if info.valid { "OK" } else { "FAILED" });
    Ok(())
}
