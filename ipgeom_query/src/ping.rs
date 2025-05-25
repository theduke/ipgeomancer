use anyhow::Result;
use std::net::IpAddr;
use std::time::{Duration, Instant};

/// IP protocol version used for network operations.
#[derive(Debug, Clone, Copy)]
pub enum IpVersion {
    /// IPv4 (A record) resolution and packets.
    V4,
    /// IPv6 (AAAA record) resolution and packets.
    V6,
}

/// Result of a single ping probe.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct PingUpdate {
    /// Sequence number of the probe starting at `0`.
    pub seq: u16,
    /// Round trip time in milliseconds. `None` if the probe timed out.
    pub rtt_ms: Option<f64>,
    /// Source IP of the reply packet.
    pub source: Option<IpAddr>,
    /// Size of the reply packet in bytes.
    pub size: Option<usize>,
    /// TTL/hop limit of the reply packet.
    pub ttl: Option<u8>,
}

/// Result of a ping operation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PingResult {
    /// IP address that was pinged.
    pub ip: IpAddr,
    /// Number of probes transmitted.
    pub transmitted: u16,
    /// Number of responses received.
    pub received: u16,
    /// Per probe results.
    pub pings: Vec<PingUpdate>,
    /// Average round trip time in milliseconds.
    pub avg_time_ms: Option<f64>,
    /// Minimum round trip time in milliseconds.
    pub min_time_ms: Option<f64>,
    /// Maximum round trip time in milliseconds.
    pub max_time_ms: Option<f64>,
    /// Standard deviation of round trip times in milliseconds.
    pub stddev_ms: Option<f64>,
    /// Total time spent for the entire ping operation in milliseconds.
    pub total_time_ms: f64,
}

/// Resolve `host` to an IP address using the system resolver.
///
/// When `version` is provided, only addresses of the corresponding
/// IP version are considered.
pub async fn resolve_host(host: &str, version: Option<IpVersion>) -> Result<IpAddr> {
    use tokio::net::lookup_host;
    let mut addrs = lookup_host((host, 0)).await?;
    let addr = match version {
        Some(IpVersion::V4) => addrs.find(|a| a.ip().is_ipv4()),
        Some(IpVersion::V6) => addrs.find(|a| a.ip().is_ipv6()),
        None => addrs.next(),
    };
    Ok(addr
        .ok_or_else(|| anyhow::anyhow!("failed to resolve host"))?
        .ip())
}

/// Ping `host` the specified number of times and report intermediate results
/// through the provided callback.
///
/// `timeout` specifies how long to wait for each probe.
/// `interval` specifies how long to wait between probes.
/// The `on_update` callback receives a [`PingUpdate`] after each probe.
pub async fn ping_with_callback<F>(
    host: &str,
    timeout: Duration,
    probes: u16,
    interval: Duration,
    interface: Option<&str>,
    version: Option<IpVersion>,
    mut on_update: F,
) -> Result<PingResult>
where
    F: FnMut(PingUpdate),
{
    use surge_ping::{Client, Config, ICMP, IcmpPacket, PingIdentifier, PingSequence};

    let ip = resolve_host(host, version).await?;

    let mut builder = Config::builder();
    if ip.is_ipv6() {
        builder = builder.kind(ICMP::V6);
    }
    if let Some(iface) = interface {
        builder = builder.interface(iface);
    }
    let config = builder.build();
    let client = Client::new(&config)?;
    let mut pinger = client.pinger(ip, PingIdentifier(0)).await;
    pinger.timeout(timeout);

    let mut recv = 0u16;
    let mut rtt_sum = 0f64;
    let mut rtt_sumsq = 0f64;
    let mut min: Option<f64> = None;
    let mut max: Option<f64> = None;
    let mut pings = Vec::with_capacity(probes as usize);
    let start = Instant::now();

    const PAYLOAD: [u8; 56] = [0; 56];

    for seq in 0..probes {
        let mut update = PingUpdate {
            seq,
            rtt_ms: None,
            source: None,
            size: None,
            ttl: None,
        };
        if let Ok((packet, rtt)) = pinger.ping(PingSequence(seq), &PAYLOAD).await {
            let ms = rtt.as_secs_f64() * 1000.0;
            update.rtt_ms = Some(ms);
            match packet {
                IcmpPacket::V4(reply) => {
                    update.source = Some(IpAddr::V4(reply.get_source()));
                    update.size = Some(reply.get_size());
                    update.ttl = reply.get_ttl();
                }
                IcmpPacket::V6(reply) => {
                    update.source = Some(IpAddr::V6(reply.get_source()));
                    update.size = Some(reply.get_size());
                    update.ttl = Some(reply.get_max_hop_limit());
                }
            }
            recv += 1;
            rtt_sum += ms;
            rtt_sumsq += ms * ms;
            min = Some(min.map_or(ms, |v| v.min(ms)));
            max = Some(max.map_or(ms, |v| v.max(ms)));
        }
        pings.push(update);
        on_update(update);
        tokio::time::sleep(interval).await;
    }

    let total_time_ms = start.elapsed().as_secs_f64() * 1000.0;
    let avg = if recv > 0 {
        Some(rtt_sum / recv as f64)
    } else {
        None
    };
    let stddev = if recv > 1 {
        let avg = rtt_sum / recv as f64;
        let var = rtt_sumsq / recv as f64 - avg * avg;
        Some(var.sqrt())
    } else {
        None
    };

    Ok(PingResult {
        ip,
        transmitted: probes,
        received: recv,
        pings,
        avg_time_ms: avg,
        min_time_ms: min,
        max_time_ms: max,
        stddev_ms: stddev,
        total_time_ms,
    })
}

/// Ping `host` the specified number of times and return statistics.
///
/// This is a convenience wrapper around [`ping_with_callback`] that does not
/// report intermediate results.
pub async fn ping(
    host: &str,
    timeout: Duration,
    probes: u16,
    interval: Duration,
    interface: Option<&str>,
    version: Option<IpVersion>,
) -> Result<PingResult> {
    ping_with_callback(host, timeout, probes, interval, interface, version, |_| {}).await
}
