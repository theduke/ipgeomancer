use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct WhoisCmd {
    /// Domain name to query
    pub domain: String,
}

pub fn handle(args: WhoisCmd) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let res = rt.block_on(ipgeom_query::domain_whois(&args.domain))?;
    println!("Server: {}\n", res.server);
    println!("{}", res.data);
    Ok(())
}
