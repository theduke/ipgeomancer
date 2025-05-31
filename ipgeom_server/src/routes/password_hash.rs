use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{ui, AppState};

#[derive(Deserialize, Default)]
pub struct Params {
    pub method: Option<String>,
    pub password: Option<String>,
}

pub struct ValidParams {
    pub method: String,
    pub password: String,
}

impl From<&ValidParams> for Params {
    fn from(v: &ValidParams) -> Self {
        Self {
            method: Some(v.method.clone()),
            password: Some(v.password.clone()),
        }
    }
}

pub(crate) fn parse_params(query: Option<&str>) -> Result<ValidParams, String> {
    let params: Params =
        serde_urlencoded::from_str(query.unwrap_or("")).map_err(|_| "invalid query parameters")?;
    let method = params.method.unwrap_or_else(|| "bcrypt".into());
    let password = params.password.unwrap_or_default();
    if password.is_empty() {
        return Err("missing 'password' parameter".into());
    }
    if method != "bcrypt" {
        return Err("unsupported method".into());
    }
    Ok(ValidParams { method, password })
}

pub async fn handler(
    State(_state): State<AppState>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let raw_params: Params =
        serde_urlencoded::from_str(query.as_deref().unwrap_or("")).unwrap_or_default();

    if query.as_deref().unwrap_or("").is_empty() {
        return ui::password_hash::page(&Params::default(), None, None);
    }

    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return ui::password_hash::page(&raw_params, None, Some(&msg));
        }
    };

    match ipgeom_query::generate_bcrypt_hash(&params.password) {
        Ok(hash) => ui::password_hash::page(&Params::from(&params), Some(&hash), None),
        Err(e) => ui::password_hash::page(&Params::from(&params), None, Some(&e.to_string())),
    }
}
