use serde::Serialize;
use std::fmt;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Result type used by [`WhoisClient`].
pub type Result<T> = std::result::Result<T, WhoisError>;

/// Errors that can occur while performing a WHOIS query.
#[derive(Debug)]
pub enum WhoisError {
    /// A network related error occurred.
    Network(std::io::Error),
    /// The operation exceeded the configured timeout.
    Timeout(&'static str),
    /// The WHOIS server reported that the requested record was not found.
    NotFound,
}

impl fmt::Display for WhoisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WhoisError::Network(e) => write!(f, "network error: {e}"),
            WhoisError::Timeout(op) => write!(f, "{} timed out", op),
            WhoisError::NotFound => write!(f, "record not found"),
        }
    }
}

impl std::error::Error for WhoisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WhoisError::Network(e) => Some(e),
            _ => None,
        }
    }
}

/// Result of a WHOIS query.
#[derive(Debug, Clone, Serialize)]
pub struct WhoisResponse {
    /// Server that returned the response.
    pub server: String,
    /// Raw WHOIS payload.
    pub data: String,
}

impl WhoisResponse {
    /// Parse key/value pairs from the raw WHOIS response.
    ///
    /// Lines starting with a key followed by a colon are interpreted as
    /// `key: value` pairs. Leading whitespace before the key is ignored.
    /// Returns `None` if no such lines are found.
    pub fn parse(&self) -> Option<Vec<(String, String)>> {
        let mut fields = Vec::new();
        for line in self.data.lines() {
            let trimmed = line.trim_start();
            if let Some((key, value)) = trimmed.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                if !key.is_empty() && !value.is_empty() {
                    fields.push((key.to_string(), value.to_string()));
                }
            }
        }
        if fields.is_empty() {
            None
        } else {
            Some(fields)
        }
    }
}

/// Asynchronous WHOIS client.
#[derive(Debug, Clone)]
pub struct WhoisClient {
    follow_referral: bool,
    timeout: Duration,
}

impl Default for WhoisClient {
    fn default() -> Self {
        Self {
            follow_referral: true,
            timeout: Duration::from_secs(10),
        }
    }
}

impl WhoisClient {
    /// Create a new client with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the read/write timeout for network operations.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Configure whether referrals should be followed.
    pub fn with_follow_referral(mut self, follow: bool) -> Self {
        self.follow_referral = follow;
        self
    }

    /// Perform a WHOIS query for `target`.
    pub async fn query(&self, target: &str) -> Result<WhoisResponse> {
        self.query_server("whois.iana.org", target).await
    }

    async fn query_server(&self, server: &str, target: &str) -> Result<WhoisResponse> {
        let data = send_query(server, target, self.timeout).await?;
        if is_not_found(&data) {
            return Err(WhoisError::NotFound);
        }
        if self.follow_referral {
            if let Some(next) = parse_referral(&data) {
                let next_data = send_query(&next, target, self.timeout).await?;
                if is_not_found(&next_data) {
                    return Err(WhoisError::NotFound);
                }
                return Ok(WhoisResponse {
                    server: next,
                    data: next_data,
                });
            }
        }
        Ok(WhoisResponse {
            server: server.to_string(),
            data,
        })
    }
}

async fn send_query(server: &str, query: &str, timeout_dur: Duration) -> Result<String> {
    let addr = if server.contains(':') {
        server.to_string()
    } else {
        format!("{server}:43")
    };
    let mut stream = timeout(timeout_dur, TcpStream::connect(addr))
        .await
        .map_err(|_| WhoisError::Timeout("connect"))?
        .map_err(WhoisError::Network)?;
    timeout(timeout_dur, stream.write_all(query.as_bytes()))
        .await
        .map_err(|_| WhoisError::Timeout("write"))?
        .map_err(WhoisError::Network)?;
    timeout(timeout_dur, stream.write_all(b"\r\n"))
        .await
        .map_err(|_| WhoisError::Timeout("write"))?
        .map_err(WhoisError::Network)?;
    let mut buf = Vec::new();
    timeout(timeout_dur, stream.read_to_end(&mut buf))
        .await
        .map_err(|_| WhoisError::Timeout("read"))?
        .map_err(WhoisError::Network)?;
    Ok(String::from_utf8_lossy(&buf).into())
}

fn parse_referral(data: &str) -> Option<String> {
    for line in data.lines() {
        if let Some(rest) = line.strip_prefix("refer:") {
            let s = rest.trim();
            if !s.is_empty() {
                return Some(s.to_string());
            }
        } else if let Some(rest) = line.strip_prefix("whois:") {
            let s = rest.trim();
            if !s.is_empty() {
                return Some(s.to_string());
            }
        }
    }
    None
}

fn is_not_found(data: &str) -> bool {
    let lower = data.to_ascii_lowercase();
    lower.contains("no match") || lower.contains("not found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn referral_parse() {
        let data = "domain: EXAMPLE\nwhois: whois.example.net\n";
        assert_eq!(parse_referral(data), Some("whois.example.net".to_string()));
    }

    #[tokio::test]
    async fn basic_query_local() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = [0u8; 32];
            let _ = socket.read(&mut buf).await.unwrap();
            socket.write_all(b"local reply\n").await.unwrap();
        });

        let client = WhoisClient::new().with_follow_referral(false);
        let resp = client
            .query_server(&addr.to_string(), "example")
            .await
            .unwrap();
        handle.await.unwrap();
        assert_eq!(resp.data.trim(), "local reply");
    }

    #[test]
    fn parse_pairs() {
        let resp = WhoisResponse {
            server: "example".into(),
            data: "Domain: EXAMPLE\n   Registrar: Example\n".into(),
        };
        let pairs = resp.parse().unwrap();
        assert_eq!(pairs[0], ("Domain".into(), "EXAMPLE".into()));
        assert_eq!(pairs[1], ("Registrar".into(), "Example".into()));
    }
}
