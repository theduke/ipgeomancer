use anyhow::{anyhow, Result};
use futures_util::{future::BoxFuture, stream::StreamExt};
use hickory_client::client::Client;
use hickory_proto::op::Query;
use hickory_proto::rr::{Name, Record, RecordType};
use hickory_proto::runtime::TokioRuntimeProvider;
use hickory_proto::udp::UdpClientStream;
use hickory_proto::xfer::{DnsHandle, DnsRequestOptions};
use std::net::{IpAddr, SocketAddr};

/// Result of a DNS query.
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// Domain name of the authoritative server that answered the query.
    pub authoritative_server: String,
    /// Records returned in the answer section.
    pub records: Vec<Record>,
}

fn parse_start_server(server: Option<&str>) -> Result<(SocketAddr, String)> {
    if let Some(s) = server {
        if let Ok(addr) = s.parse() {
            Ok((addr, s.to_string()))
        } else {
            let with_port = format!("{}:53", s);
            let addr = with_port
                .parse()
                .map_err(|_| anyhow!("invalid server address"))?;
            Ok((addr, s.to_string()))
        }
    } else {
        Ok((
            SocketAddr::new(IpAddr::from([198, 41, 0, 4]), 53),
            "a.root-servers.net".into(),
        ))
    }
}

async fn send_query(
    server: SocketAddr,
    name: &Name,
    record_type: RecordType,
    recursion_desired: bool,
) -> Result<hickory_proto::xfer::DnsResponse> {
    let conn = UdpClientStream::builder(server, TokioRuntimeProvider::default()).build();
    let (client, bg) = Client::connect(conn).await?;
    tokio::spawn(bg);

    let query = Query::query(name.clone(), record_type);
    let mut opts = DnsRequestOptions::default();
    opts.recursion_desired = recursion_desired;
    opts.use_edns = true;
    let mut send = client.lookup(query, opts);
    let response = send.next().await.ok_or_else(|| anyhow!("no response"))??;
    Ok(response)
}

fn lookup_ns_ip(ns: Name) -> BoxFuture<'static, Result<IpAddr>> {
    Box::pin(async move {
        for ty in [RecordType::A, RecordType::AAAA] {
            if let Ok(resp) = authoritative_query(&ns.to_utf8(), ty, None).await {
                for rec in resp.records {
                    if let Some(a) = rec.data().as_a() {
                        return Ok(IpAddr::V4(a.0));
                    }
                    if let Some(a) = rec.data().as_aaaa() {
                        return Ok(IpAddr::V6(a.0));
                    }
                }
            }
        }
        Err(anyhow!("no address for name server"))
    })
}

async fn find_authoritative_server(
    name: &Name,
    server: Option<&str>,
) -> Result<(String, SocketAddr)> {
    let (mut current_addr, mut current_name) = parse_start_server(server)?;

    for _ in 0..16 {
        let resp = send_query(current_addr, name, RecordType::NS, false).await?;
        if resp.authoritative() {
            return Ok((current_name, current_addr));
        }

        let mut next = None;
        for ns in resp
            .name_servers()
            .iter()
            .filter(|r| r.record_type() == RecordType::NS)
        {
            if let Some(ns_name) = ns.data().as_ns() {
                if let Some(addr) = resp.additionals().iter().find_map(|r| {
                    if r.name() == &ns_name.0 {
                        if let Some(a) = r.data().as_a() {
                            return Some(IpAddr::V4(a.0));
                        }
                        if let Some(a) = r.data().as_aaaa() {
                            return Some(IpAddr::V6(a.0));
                        }
                    }
                    None
                }) {
                    next = Some((ns_name.0.to_utf8(), SocketAddr::new(addr, 53)));
                    break;
                } else if let Ok(addr) = lookup_ns_ip(ns_name.0.clone()).await {
                    next = Some((ns_name.0.to_utf8(), SocketAddr::new(addr, 53)));
                    break;
                }
            }
        }

        if let Some((n, addr)) = next {
            current_name = n;
            current_addr = addr;
        } else {
            return Err(anyhow!("failed to resolve next name server"));
        }
    }

    Err(anyhow!("too many redirects"))
}

/// Perform an iterative DNS lookup following the chain of authoritative servers.
///
/// `server` specifies the DNS server to start the lookup with. If `None`, the
/// query starts at one of the root servers.
pub async fn authoritative_query(
    name: &str,
    record_type: RecordType,
    server: Option<&str>,
) -> Result<QueryResult> {
    let fqdn = if name.ends_with('.') {
        Name::from_ascii(name)?
    } else {
        Name::from_ascii(format!("{name}."))?
    };

    let (auth_name, auth_addr) = find_authoritative_server(&fqdn, server).await?;
    let resp = send_query(auth_addr, &fqdn, record_type, false).await?;
    if !resp.authoritative() {
        return Err(anyhow!("unexpected non-authoritative response"));
    }
    Ok(QueryResult {
        authoritative_server: auth_name,
        records: resp.answers().to_vec(),
    })
}
