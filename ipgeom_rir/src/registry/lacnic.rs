use crate::{Client, RirProvider};

#[derive(Debug, Clone, Copy)]
pub struct Lacnic {}

impl Lacnic {
    const RPSL_DOWNLOAD_URL: &'static str = "https://ftp.lacnic.net/pub/dbase/lacnic.db.gz";
}

impl RirProvider for Lacnic {
    fn build_rpsl_db_request(&self, client: &Client) -> reqwest::blocking::RequestBuilder {
        client.get(Self::RPSL_DOWNLOAD_URL)
    }
}

#[cfg(test)]
mod tests {}
