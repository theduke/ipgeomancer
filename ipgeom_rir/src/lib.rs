mod types;

mod registry;

type Client = reqwest::Client;

pub trait Rir: std::fmt::Debug + Send + Sync {
    /// Download the latest dump of the RPSL database from the RIR.
    fn download_rpsl_db(
        &self,
        client: &Client,
    ) -> impl Future<Output = Result<DbData, anyhow::Error>> + Send;
}

pub struct DbData {
    /// If the data is gzip-compressed.
    pub gzip: bool,
    pub reader: Box<dyn std::io::Read + Send + Sync>,
}
