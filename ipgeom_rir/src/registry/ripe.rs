use crate::{Client, RirProvider};

#[derive(Debug, Clone, Copy)]
pub struct Ripe {}

impl Ripe {
    const RPSL_DOWNLOAD_URL: &'static str = "https://ftp.ripe.net/ripe/dbase/ripe.db.gz";
}

impl RirProvider for Ripe {
    fn build_rpsl_db_request(&self, client: &Client) -> reqwest::blocking::RequestBuilder {
        client.get(Self::RPSL_DOWNLOAD_URL)
    }
}

#[cfg(test)]
mod tests {}
