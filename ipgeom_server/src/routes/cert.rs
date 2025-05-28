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
    let raw_params: Params =
        serde_urlencoded::from_str(query.as_deref().unwrap_or("")).unwrap_or_default();

    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => return ui::cert::page(raw_params.domain.as_deref(), None, Some(&msg)),
    };

    match ipgeom_query::fetch_certificate(&params.domain).await {
        Ok(info) => ui::cert::page(Some(&params.domain), Some(&info), None),
        Err(e) => ui::cert::page(Some(&params.domain), None, Some(&e.to_string())),
    }
}
