use anyhow::{anyhow, Result};
use std::sync::Arc;

/// Result of checking a domain's TLS certificate.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CertificateInfo {
    /// Subject of the certificate.
    pub subject: String,
    /// Issuer of the certificate.
    pub issuer: String,
    /// Not before validity timestamp.
    pub not_before: String,
    /// Not after validity timestamp.
    pub not_after: String,
    /// Whether the certificate was validated successfully.
    pub valid: bool,
}

/// Fetch and validate the TLS certificate for `domain`.
pub async fn fetch_certificate(domain: &str) -> Result<CertificateInfo> {
    let domain = domain.to_string();
    tokio::task::spawn_blocking(move || fetch_certificate_blocking(&domain)).await?
}

fn fetch_certificate_blocking(domain: &str) -> Result<CertificateInfo> {
    use rustls::pki_types::ServerName;
    use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
    use std::net::TcpStream;
    use std::time::Duration;

    let root_store = RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let server_name =
        ServerName::try_from(domain.to_string()).map_err(|_| anyhow!("invalid domain"))?;
    let addr = format!("{}:443", domain);
    let stream = TcpStream::connect(addr)?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

    let conn = ClientConnection::new(Arc::new(config), server_name)?;
    let mut tls = StreamOwned::new(conn, stream);
    // complete handshake
    while tls.conn.is_handshaking() {
        tls.conn.complete_io(&mut tls.sock)?;
    }

    let certs = tls
        .conn
        .peer_certificates()
        .ok_or_else(|| anyhow!("no certificate"))?;
    let cert = certs.first().ok_or_else(|| anyhow!("no certificate"))?;

    use x509_parser::prelude::*;
    let (_, parsed) = parse_x509_certificate(cert.as_ref())
        .map_err(|e| anyhow!(format!("failed to parse certificate: {e}")))?;

    let info = CertificateInfo {
        subject: parsed.subject().to_string(),
        issuer: parsed.issuer().to_string(),
        not_before: parsed.validity().not_before.to_string(),
        not_after: parsed.validity().not_after.to_string(),
        valid: true,
    };
    Ok(info)
}
