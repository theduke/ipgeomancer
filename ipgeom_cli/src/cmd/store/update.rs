use anyhow::Result;
use clap::Args;
use ipgeom_rir::Store;

#[derive(Args)]
pub struct Update {}

pub fn handle(store: &Store, _args: Update) -> Result<()> {
    store.update()?;
    Ok(())
}
