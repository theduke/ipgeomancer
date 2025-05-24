use crate::{Client, RirProvider};

#[derive(Debug, Clone, Copy)]
pub struct Apnic {}

impl Apnic {
    const RPSL_DOWNLOAD_URL: &'static str = "https://ftp.apnic.net/apnic/dbase/data/apnic.db.gz";
}

impl RirProvider for Apnic {
    fn build_rpsl_db_request(&self, client: &Client) -> reqwest::blocking::RequestBuilder {
        client.get(Self::RPSL_DOWNLOAD_URL)
    }
}

#[cfg(test)]
mod tests {}
