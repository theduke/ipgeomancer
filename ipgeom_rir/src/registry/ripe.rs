use std::io::Read as _;
use std::path::PathBuf;

use anyhow::Context;

use crate::{Client, DbData, RirProvider};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Copy)]
pub struct Ripe {}

impl Ripe {
    const RPSL_DOWNLOAD_URL: &'static str = "https://ftp.ripe.net/ripe/dbase/ripe.db.gz";
}

impl RirProvider for Ripe {
    fn download_rpsl_db<'a>(
        &'a self,
        client: &'a Client,
    ) -> Pin<Box<dyn Future<Output = Result<DbData, anyhow::Error>> + Send + 'a>> {
        Box::pin(async move {
            let tmp_path = PathBuf::from("/tmp/ripe.db.gz");
            let body = if !tmp_path.exists() {
                let res = client
                    .get(Self::RPSL_DOWNLOAD_URL)
                    // .header(reqwest::header::ACCEPT_ENCODING, "gzip")
                    .send()
                    .await?
                    .error_for_status()?;

                let body: Vec<u8> = res.bytes().await?.into();
                tokio::fs::write(&tmp_path, &body).await?;
                tracing::debug!("Downloaded RIR data to {}", tmp_path.display());
                body
            } else {
                tokio::fs::read(&tmp_path).await?
            };

            tracing::debug!("Decoding gzipped RIR data from {}", tmp_path.display());
            let mut output = String::new();
            flate2::read::MultiGzDecoder::new(body.as_slice())
                .read_to_string(&mut output)
                .context("failed to decode gzipped RIR data")?;

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
