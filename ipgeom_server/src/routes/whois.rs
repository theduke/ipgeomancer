use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{ui, AppState};

#[derive(Deserialize, Default)]
pub struct Params {
    pub domain: Option<String>,
}

pub struct ValidParams {
    pub domain: String,
}

impl From<&ValidParams> for Params {
    fn from(v: &ValidParams) -> Self {
        Self {
            domain: Some(v.domain.clone()),
        }
    }
}

pub(crate) fn parse_params(query: Option<&str>) -> Result<ValidParams, String> {
    let params: Params =
        serde_urlencoded::from_str(query.unwrap_or("")).map_err(|_| "invalid query parameters")?;
    let domain = params.domain.unwrap_or_default();
    if domain.trim().is_empty() {
        return Err("missing 'domain' parameter".into());
    }
    Ok(ValidParams { domain })
}

pub async fn handler(
    State(_state): State<AppState>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let raw_params: Params =
        serde_urlencoded::from_str(query.as_deref().unwrap_or("")).unwrap_or_default();

    if query.as_deref().unwrap_or("").is_empty() {
        return ui::whois::page(&Params::default(), None, None);
    }

    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => return ui::whois::page(&raw_params, None, Some(&msg)),
    };

    match ipgeom_query::domain_whois(&params.domain).await {
        Ok(json) => ui::whois::page(&Params::from(&params), Some(&json), None),
        Err(e) => ui::whois::page(&Params::from(&params), None, Some(&e.to_string())),
    }
}
