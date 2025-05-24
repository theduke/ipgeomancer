use crate::{Client, RirProvider};

#[derive(Debug, Clone, Copy)]
pub struct Afrinic {}

impl Afrinic {
    const RPSL_DOWNLOAD_URL: &'static str = "https://ftp.afrinic.net/pub/dbase/afrinic.db.gz";
}

impl RirProvider for Afrinic {
    fn build_rpsl_db_request(&self, client: &Client) -> reqwest::blocking::RequestBuilder {
        client.get(Self::RPSL_DOWNLOAD_URL)
    }
}

#[cfg(test)]
mod tests {}
