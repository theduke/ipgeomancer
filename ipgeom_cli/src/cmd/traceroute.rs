use anyhow::Result;
use async_traceroute::ProbeMethod;
use clap::Args;
use ipgeom_query::IpVersion;
use std::time::Duration;

/// Trace the route to a host.
#[derive(Args)]
pub struct TracerouteCmd {
    /// Set the max number of hops
    #[arg(short = 'm', long, default_value_t = 30)]
    max_hops: u8,
    /// Set the number of probes per hop
    #[arg(short = 'q', long, default_value_t = 3)]
    queries: u16,
    /// Wait time in seconds for each probe
    #[arg(short = 'w', long, default_value_t = 3)]
    wait: u64,
    /// Number of probes to send simultaneously
    #[arg(short = 'N', long, default_value_t = 16)]
    sim_queries: u16,
    /// Probe method (udp, tcp, icmp)
    #[arg(short = 'P', long, value_enum, default_value_t = ProbeMethod::UDP)]
    probe_method: ProbeMethod,
    /// Port/sequence number depending on probe method
    #[arg(short = 'p', long)]
    port: Option<u16>,
    /// Perform reverse DNS lookups
    #[arg(short = 'n', default_value_t = true)]
    dns_lookup: bool,
    /// Use IPv4 instead of IPv6
    #[arg(long, conflicts_with = "ipv6")]
    ipv4: bool,
    /// Use IPv6 instead of IPv4
    #[arg(long, conflicts_with = "ipv4")]
    ipv6: bool,
    /// Network interface to use
    #[arg(short = 'i', long)]
    interface: Option<String>,
    /// Host to trace
    host: String,
}

pub async fn handle(args: TracerouteCmd) -> Result<()> {
    let version = if args.ipv4 {
        Some(IpVersion::V4)
    } else if args.ipv6 {
        Some(IpVersion::V6)
    } else {
        None
    };

    let ip = ipgeom_query::resolve_host(&args.host, version).await?;
    println!(
        "traceroute to {} ({}) , {} hops max",
        args.host, ip, args.max_hops
    );

    let res = ipgeom_query::traceroute(
        &args.host,
        args.probe_method,
        args.max_hops,
        args.queries,
        Duration::from_secs(args.wait),
        args.sim_queries,
        args.port,
        args.dns_lookup,
        args.interface.as_deref(),
        version,
    )
    .await?;

    for hop in res.hops {
        print!("{:>2}  ", hop.ttl);
        let mut last_addr: Option<String> = None;
        let mut last_host: Option<String> = None;
        for probe in hop.probes {
            if let Some(rtt) = probe.rtt_ms {
                let addr = probe
                    .address
                    .map(|ip| ip.to_string())
                    .unwrap_or_else(|| "*".into());
                let host = probe.hostname.clone().unwrap_or_else(|| addr.clone());
                if last_addr.as_deref() != Some(&addr) || last_host.as_deref() != Some(&host) {
                    print!("{} ({})  ", host, addr);
                    last_addr = Some(addr);
                    last_host = Some(host);
                }
                print!("{:.3} ms  ", rtt);
            } else {
                print!("* ");
            }
        }
        println!();
    }

    Ok(())
}
