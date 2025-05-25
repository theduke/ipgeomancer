use anyhow::Result;
use clap::Args;
use ipgeom_query::IpVersion;
use std::time::Duration;

/// Send ICMP echo requests to a host.
#[derive(Args)]
pub struct PingCmd {
    /// Timeout in seconds for each probe
    #[arg(long, default_value_t = 5)]
    timeout: u64,
    /// Number of probes to send
    #[arg(long, default_value_t = 4)]
    probes: u16,
    /// Interval in seconds between probes
    #[arg(long, default_value_t = 1)]
    interval: u64,
    /// Use IPv4 instead of IPv6
    #[arg(long, conflicts_with = "ipv6")]
    ipv4: bool,
    /// Use IPv6 instead of IPv4
    #[arg(long, conflicts_with = "ipv4")]
    ipv6: bool,
    /// Network interface to use
    #[arg(short = 'i', long)]
    interface: Option<String>,
    /// Host to ping
    host: String,
}

pub fn handle(args: PingCmd) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;

    let version = if args.ipv4 {
        Some(IpVersion::V4)
    } else if args.ipv6 {
        Some(IpVersion::V6)
    } else {
        None
    };

    let ip = rt.block_on(ipgeom_query::resolve_host(&args.host, version))?;
    println!("PING {} ({}) 56(84) bytes of data.", args.host, ip);

    let res = rt.block_on(ipgeom_query::ping_with_callback(
        &args.host,
        Duration::from_secs(args.timeout),
        args.probes,
        Duration::from_secs(args.interval),
        args.interface.as_deref(),
        version,
        |update| {
            if let Some(rtt) = update.rtt_ms {
                let size = update.size.unwrap_or(0);
                let ttl = update
                    .ttl
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "?".into());
                let src = update
                    .source
                    .map(|ip| ip.to_string())
                    .unwrap_or_else(|| "".into());
                println!(
                    "{} bytes from {}: icmp_seq={} ttl={} time={:.2} ms",
                    size,
                    src,
                    update.seq + 1,
                    ttl,
                    rtt
                );
            } else {
                println!("icmp_seq={} timed out", update.seq + 1);
            }
        },
    ))?;

    let loss = (res.transmitted - res.received) as f64 / res.transmitted as f64 * 100.0;
    println!("--- {} ping statistics ---", args.host);
    println!(
        "{} packets transmitted, {} received, {:.0}% packet loss, time {}ms",
        res.transmitted, res.received, loss, res.total_time_ms as u64
    );
    if let (Some(min), Some(avg), Some(max), Some(dev)) = (
        res.min_time_ms,
        res.avg_time_ms,
        res.max_time_ms,
        res.stddev_ms,
    ) {
        println!(
            "rtt min/avg/max/mdev = {:.3}/{:.3}/{:.3}/{:.3} ms",
            min, avg, max, dev
        );
    }
    Ok(())
}
