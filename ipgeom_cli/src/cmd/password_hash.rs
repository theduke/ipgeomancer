use anyhow::Result;
use clap::Args;

/// Generate a password hash using the specified method.
#[derive(Args)]
pub struct MakePasswordHashCmd {
    /// Hashing method to use (only 'bcrypt' supported)
    #[arg(long, default_value = "bcrypt")]
    method: String,
    /// Password to hash
    password: String,
}

pub fn handle(args: MakePasswordHashCmd) -> Result<()> {
    match args.method.as_str() {
        "bcrypt" => {
            let hash = ipgeom_query::generate_bcrypt_hash(&args.password)?;
            println!("{}", hash);
            Ok(())
        }
        _ => Err(anyhow::anyhow!("unsupported method")),
    }
}
