use anyhow::{Result, anyhow};
use async_traceroute::{ProbeMethod, TracerouteBuilder};
use futures_util::StreamExt;
use std::{collections::BTreeMap, net::IpAddr, time::Duration};

/// Result of a single traceroute probe.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TracerouteUpdate {
    /// TTL/hop that was probed.
    pub ttl: u8,
    /// Sequence number of the probe within the TTL starting at `0`.
    pub seq: u16,
    /// IP address of the responding hop.
    pub address: Option<IpAddr>,
    /// Hostname of the responding hop if reverse DNS succeeded.
    pub hostname: Option<String>,
    /// Round trip time in milliseconds. `None` if the probe timed out.
    pub rtt_ms: Option<f64>,
}

/// Result of a single hop.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TracerouteHop {
    /// Hop number / TTL value.
    pub ttl: u8,
    /// Probes performed for this hop.
    pub probes: Vec<TracerouteUpdate>,
}

/// Full traceroute result.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TracerouteResult {
    /// Destination IP address.
    pub destination: IpAddr,
    /// Maximum TTL that was probed.
    pub max_ttl: u8,
    /// Number of queries per hop.
    pub queries_per_hop: u16,
    /// Hops encountered during the trace.
    pub hops: Vec<TracerouteHop>,
}

/// Perform a traceroute and report intermediate results through `on_update`.
#[allow(clippy::too_many_arguments)]
pub async fn traceroute_with_callback<F>(
    host: &str,
    probe_method: ProbeMethod,
    max_ttl: u8,
    queries_per_hop: u16,
    wait_time: Duration,
    simultaneous_queries: u16,
    port: Option<u16>,
    dns_lookup: bool,
    interface: Option<&str>,
    version: Option<crate::IpVersion>,
    mut on_update: F,
) -> Result<TracerouteResult>
where
    F: FnMut(TracerouteUpdate) + Send,
{
    let ip_addr = super::resolve_host(host, version).await?;

    let traceroute = match probe_method {
        ProbeMethod::UDP => {
            let mut b = TracerouteBuilder::udp()
                .destination_address(ip_addr)
                .max_ttl(max_ttl)
                .queries_per_hop(queries_per_hop)
                .simultaneous_queries(simultaneous_queries)
                .max_wait_probe(wait_time)
                .active_dns_lookup(dns_lookup)
                .initial_destination_port(port.unwrap_or(33434));
            if let Some(iface) = interface {
                b = b.network_interface(iface);
            }
            b.build()
        }
        ProbeMethod::TCP => {
            let mut b = TracerouteBuilder::tcp()
                .destination_address(ip_addr)
                .max_ttl(max_ttl)
                .queries_per_hop(queries_per_hop)
                .simultaneous_queries(simultaneous_queries)
                .max_wait_probe(wait_time)
                .active_dns_lookup(dns_lookup)
                .initial_destination_port(port.unwrap_or(80));
            if let Some(iface) = interface {
                b = b.network_interface(iface);
            }
            b.build()
        }
        ProbeMethod::ICMP => {
            let mut b = TracerouteBuilder::icmp()
                .destination_address(ip_addr)
                .max_ttl(max_ttl)
                .queries_per_hop(queries_per_hop)
                .simultaneous_queries(simultaneous_queries)
                .max_wait_probe(wait_time)
                .active_dns_lookup(dns_lookup)
                .initial_sequence_number(port.unwrap_or(1));
            if let Some(iface) = interface {
                b = b.network_interface(iface);
            }
            b.build()
        }
    };

    let traceroute = traceroute.map_err(|e| anyhow!(e))?;
    let stream = traceroute.trace();
    futures_util::pin_mut!(stream);
    let mut hops: BTreeMap<u8, Vec<TracerouteUpdate>> = BTreeMap::new();
    while let Some(res) = stream.next().await {
        match res {
            Ok(probe) => {
                let ttl = probe.ttl();
                let entry = hops.entry(ttl).or_default();
                let seq = entry.len() as u16;
                let update = TracerouteUpdate {
                    ttl,
                    seq,
                    address: Some(IpAddr::V4(probe.from_address())),
                    hostname: probe.get_hostname(),
                    rtt_ms: Some(probe.rtt().as_secs_f64() * 1000.0),
                };
                entry.push(update.clone());
                on_update(update);
            }
            Err(err) => {
                let ttl = err.get_ttl();
                let entry = hops.entry(ttl).or_default();
                let seq = entry.len() as u16;
                let update = TracerouteUpdate {
                    ttl,
                    seq,
                    address: None,
                    hostname: None,
                    rtt_ms: None,
                };
                entry.push(update.clone());
                on_update(update);
            }
        }
    }

    let hops = hops
        .into_iter()
        .map(|(ttl, probes)| TracerouteHop { ttl, probes })
        .collect::<Vec<_>>();

    Ok(TracerouteResult {
        destination: ip_addr,
        max_ttl,
        queries_per_hop,
        hops,
    })
}

#[allow(clippy::too_many_arguments)]
pub async fn traceroute(
    host: &str,
    probe_method: ProbeMethod,
    max_ttl: u8,
    queries_per_hop: u16,
    wait_time: Duration,
    simultaneous_queries: u16,
    port: Option<u16>,
    dns_lookup: bool,
    interface: Option<&str>,
    version: Option<crate::IpVersion>,
) -> Result<TracerouteResult> {
    traceroute_with_callback(
        host,
        probe_method,
        max_ttl,
        queries_per_hop,
        wait_time,
        simultaneous_queries,
        port,
        dns_lookup,
        interface,
        version,
        |_| {},
    )
    .await
}
