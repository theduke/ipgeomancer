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
    let params: Params =
        serde_urlencoded::from_str(query.as_deref().unwrap_or("")).unwrap_or_default();
    match parse_params(query.as_deref()) {
        Ok(valid) => match ipgeom_query::domain_whois(&valid.domain).await {
            Ok(json) => ui::whois::page(Some(&valid.domain), Some(&json), None),
            Err(e) => ui::whois::page(Some(&valid.domain), None, Some(&e.to_string())),
        },
        Err(msg) => ui::whois::page(params.domain.as_deref(), None, Some(&msg)),
    }
}
