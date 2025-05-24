use std::io::Read as _;

use crate::{Client, Rir};

#[derive(Debug, Clone, Copy)]
pub struct Afrinic {}

impl Afrinic {
    const RPSL_DOWNLOAD_URL: &'static str = "https://ftp.afrinic.net/pub/dbase/afrinic.db.gz";
}

impl Rir for Afrinic {
    async fn download_rpsl_db(&self, client: &Client) -> Result<String, anyhow::Error> {
        let res = client
            .get(Self::RPSL_DOWNLOAD_URL)
            .header(reqwest::header::ACCEPT_ENCODING, "gzip")
            .send()
            .await?
            .error_for_status()?;

        let body: Vec<u8> = res.bytes().await?.into();
        let reader = std::io::Cursor::new(body);

        let mut output = String::new();
        flate2::read::GzDecoder::new(reader).read_to_string(&mut output)?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {}
