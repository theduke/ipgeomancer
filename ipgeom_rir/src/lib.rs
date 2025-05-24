mod store;
mod types;

use std::pin::Pin;

pub use {self::store::Store, self::types::Rir as RirKind};

mod registry;

type Client = reqwest::Client;

pub trait Rir: std::fmt::Debug + Send + Sync {
    /// Download the latest dump of the RPSL database from the RIR.
    fn download_rpsl_db<'a>(
        &'a self,
        client: &'a Client,
    ) -> Pin<Box<dyn Future<Output = Result<DbData, anyhow::Error>> + Send + 'a>>;
}

pub struct DbData {
    /// If the data is gzip-compressed.
    pub gzip: bool,
    pub reader: Box<dyn std::io::Read + Send + Sync>,
}
