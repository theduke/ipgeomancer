use crate::{Client, RirProvider};

#[derive(Debug, Clone, Copy)]
pub struct Arin {}

impl Arin {
    const RPSL_DOWNLOAD_URL: &'static str = "https://ftp.arin.net/pub/rr/arin.db.gz";
}

impl RirProvider for Arin {
    fn build_rpsl_db_request(&self, client: &Client) -> reqwest::blocking::RequestBuilder {
        client.get(Self::RPSL_DOWNLOAD_URL)
    }
}

#[cfg(test)]
mod tests {}
