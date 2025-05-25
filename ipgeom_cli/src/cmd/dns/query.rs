use anyhow::{Result, anyhow};
use clap::Args;
use std::str::FromStr;

use hickory_proto::rr::RecordType;

#[derive(Args)]
pub struct QueryCmd {
    /// DNS server to query (optional)
    #[arg(short, long)]
    pub server: Option<String>,
    /// Record type to query (e.g. A, AAAA, MX)
    #[arg(short = 't', long)]
    pub record_type: String,
    /// Name to query
    pub name: String,
}

pub fn handle(args: QueryCmd) -> Result<()> {
    let record_type =
        RecordType::from_str(&args.record_type).map_err(|_| anyhow!("invalid record type"))?;

    let rt = tokio::runtime::Runtime::new()?;
    let res = rt.block_on(ipgeom_query::dns::authoritative_query(
        &args.name,
        record_type,
        args.server.as_deref(),
    ))?;

    println!("Authoritative server: {}", res.authoritative_server);
    if res.records.is_empty() {
        println!("No records found");
    } else {
        for rec in res.records {
            println!(
                "{} {} {} {}",
                rec.name(),
                rec.ttl(),
                rec.record_type(),
                rec.data()
            );
        }
    }
    Ok(())
}
