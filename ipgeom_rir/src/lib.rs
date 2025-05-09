// mod rpsl;
mod types;

mod registry;

type Client = reqwest::Client;

pub trait Rir: std::fmt::Debug + Send + Sync {
    /// Download the latest dump of the RPSL database from the RIR.
    fn download_rpsl_db(
        &self,
        client: &Client,
    ) -> impl Future<Output = Result<String, anyhow::Error>> + Send;
}
