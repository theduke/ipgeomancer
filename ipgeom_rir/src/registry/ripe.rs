use std::io::Read as _;

use crate::{Client, DbData, Rir};

#[derive(Debug, Clone, Copy)]
pub struct Ripe {}

impl Ripe {
    const RPSL_DOWNLOAD_URL: &'static str = "https://ftp.ripe.net/ripe/dbase/ripe.db.gz";
}

impl Rir for Ripe {
    async fn download_rpsl_db(&self, client: &Client) -> Result<DbData, anyhow::Error> {
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

        let data = DbData {
            gzip: false,
            reader: Box::new(std::io::Cursor::new(output)),
        };

        Ok(data)
    }
}

#[cfg(test)]
mod tests {}
