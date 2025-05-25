pub mod cert;
pub mod dns;
pub mod ping;
pub mod traceroute;

pub use cert::{CertificateInfo, fetch_certificate};
pub use ping::{IpVersion, PingResult, PingUpdate, ping, ping_with_callback, resolve_host};
pub use traceroute::{
    TracerouteHop, TracerouteResult, TracerouteUpdate, traceroute, traceroute_with_callback,
};

use anyhow::{Result, anyhow};

/// Perform a generic RDAP query using `icann_rdap_client`.
pub async fn rdap(
    query: icann_rdap_client::rdap::QueryType,
) -> Result<icann_rdap_client::rdap::ResponseData> {
    let config = icann_rdap_client::http::ClientConfig::default();
    let client = icann_rdap_client::http::create_client(&config)?;
    let store = icann_rdap_client::iana::MemoryBootstrapStore::new();
    let response =
        icann_rdap_client::rdap::rdap_bootstrapped_request(&query, &client, &store, |_| ()).await?;
    Ok(response)
}

/// Perform a domain WHOIS query using the RDAP protocol.
pub async fn domain_whois_rdap(
    query: icann_rdap_client::rdap::QueryType,
) -> Result<icann_rdap_client::rdap::ResponseData> {
    let config = icann_rdap_client::http::ClientConfig::default();
    let client = icann_rdap_client::http::create_client(&config)?;
    let store = icann_rdap_client::iana::MemoryBootstrapStore::new();
    let response =
        icann_rdap_client::rdap::rdap_bootstrapped_request(&query, &client, &store, |_| ()).await?;
    Ok(response)
}

/// Perform a domain WHOIS query using the traditional WHOIS protocol.
pub async fn domain_whois(domain: &str) -> Result<ipgeom_whois::WhoisResponse> {
    ipgeom_whois::WhoisClient::default()
        .query(domain)
        .await
        .map_err(|e| anyhow!(e))
}
