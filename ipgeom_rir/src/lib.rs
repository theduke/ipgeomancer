mod db;
mod store;
mod types;

use std::io::Read;

pub use {
    self::db::{Database, sqlite::SqliteDb},
    self::store::Store,
    self::types::Rir as RirKind,
};

mod registry;

type Client = reqwest::blocking::Client;

pub trait RirProvider: std::fmt::Debug + Send + Sync {
    /// Build the request used to download the latest dump of the RPSL database
    /// from the RIR.
    fn build_rpsl_db_request(&self, client: &Client) -> reqwest::blocking::RequestBuilder;

    /// Download the latest dump of the RPSL database from the RIR.
    ///
    /// The default implementation sends the request built by
    /// [`build_rpsl_db_request`] and, if the response body appears to be
    /// gzip-compressed based on the `Content-Type` header, transparently
    /// decompresses it.
    fn download_rpsl_db(&self, client: &Client) -> Result<DbData, anyhow::Error> {
        let req = self
            .build_rpsl_db_request(client)
            .header(reqwest::header::ACCEPT_ENCODING, "gzip")
            .build()?;

        let uri_is_gzip = req.url().path().ends_with(".gz");

        let res = client.execute(req)?.error_for_status()?;

        let reader: Box<dyn Read + Send + Sync> = if uri_is_gzip {
            Box::new(flate2::read::MultiGzDecoder::new(res))
        } else {
            Box::new(res)
        };

        Ok(DbData {
            gzip: false,
            reader,
        })
    }
}

pub struct DbData {
    /// If the data is gzip-compressed.
    pub gzip: bool,
    pub reader: Box<dyn Read + Send + Sync>,
}
