use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct WhoisCmd {
    /// Domain name to query
    pub domain: String,
}

pub async fn handle(args: WhoisCmd) -> Result<()> {
    let res = ipgeom_query::domain_whois(&args.domain).await?;
    println!("Server: {}\n", res.server);
    println!("{}", res.data);
    Ok(())
}
