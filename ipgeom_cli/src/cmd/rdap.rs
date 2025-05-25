use anyhow::Result;
use clap::{Args, ValueEnum};
use std::str::FromStr;

#[derive(Copy, Clone, ValueEnum)]
pub enum RdapType {
    Ipv4Addr,
    Ipv6Addr,
    Ipv4Cidr,
    Ipv6Cidr,
    AsNumber,
    Domain,
    ALabel,
    Nameserver,
    Entity,
    DomainNsIpSearch,
    NameserverIpSearch,
    Url,
    Help,
}

impl RdapType {
    fn build(self, value: &str) -> Result<icann_rdap_client::rdap::QueryType> {
        use icann_rdap_client::rdap::QueryType;
        let qt = match self {
            RdapType::Ipv4Addr => QueryType::ipv4(value)?,
            RdapType::Ipv6Addr => QueryType::ipv6(value)?,
            RdapType::Ipv4Cidr => QueryType::ipv4cidr(value)?,
            RdapType::Ipv6Cidr => QueryType::ipv6cidr(value)?,
            RdapType::AsNumber => QueryType::autnum(value)?,
            RdapType::Domain => QueryType::domain(value)?,
            RdapType::ALabel => QueryType::alabel(value)?,
            RdapType::Nameserver => QueryType::ns(value)?,
            RdapType::Entity => QueryType::Entity(value.to_string()),
            RdapType::DomainNsIpSearch => QueryType::domain_ns_ip_search(value)?,
            RdapType::NameserverIpSearch => QueryType::ns_ip_search(value)?,
            RdapType::Url => QueryType::Url(value.to_string()),
            RdapType::Help => QueryType::Help,
        };
        Ok(qt)
    }
}

#[derive(Args)]
pub struct RdapCmd {
    /// Query type
    #[arg(short = 't', long, value_enum)]
    pub query_type: Option<RdapType>,
    /// Query string
    pub query: String,
}

pub fn handle(args: RdapCmd) -> Result<()> {
    let query_type = if let Some(qt) = args.query_type {
        qt.build(&args.query)?
    } else {
        icann_rdap_client::rdap::QueryType::from_str(&args.query)?
    };

    let rt = tokio::runtime::Runtime::new()?;
    let res = rt.block_on(ipgeom_query::rdap(query_type))?;
    println!("{}", serde_json::to_string_pretty(&res)?);
    Ok(())
}
