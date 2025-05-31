use anyhow::Result;

/// Generate a bcrypt password hash using the default cost.
///
/// This format is compatible with tools like `htpasswd` and can be
/// used by web servers such as Apache or nginx.
pub fn generate_bcrypt_hash(password: &str) -> Result<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| e.into())
}
