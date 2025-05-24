use std::io::Read as _;

use crate::{Client, DbData, Rir};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Copy)]
pub struct Lacnic {}

impl Lacnic {
    const RPSL_DOWNLOAD_URL: &'static str = "https://ftp.lacnic.net/pub/dbase/lacnic.db.gz";
}

impl Rir for Lacnic {
    fn download_rpsl_db<'a>(
        &'a self,
        client: &'a Client,
    ) -> Pin<Box<dyn Future<Output = Result<DbData, anyhow::Error>> + Send + 'a>> {
        Box::pin(async move {
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
        })
    }
}

#[cfg(test)]
mod tests {}
