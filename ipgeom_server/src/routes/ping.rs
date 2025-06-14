use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
};
use serde::Deserialize;
use std::time::Duration;

use crate::{ui, AppState};

#[derive(Deserialize, Default)]
pub struct Params {
    pub host: Option<String>,
    pub timeout: Option<u64>,
    pub probes: Option<u16>,
    pub interval: Option<u64>,
}

pub struct ValidParams {
    pub host: String,
    pub timeout: u64,
    pub probes: u16,
    pub interval: u64,
}

impl From<&ValidParams> for Params {
    fn from(v: &ValidParams) -> Self {
        Self {
            host: Some(v.host.clone()),
            timeout: Some(v.timeout),
            probes: Some(v.probes),
            interval: Some(v.interval),
        }
    }
}

pub(crate) fn parse_params(query: Option<&str>) -> Result<ValidParams, String> {
    let params: Params =
        serde_urlencoded::from_str(query.unwrap_or("")).map_err(|_| "invalid query parameters")?;
    let host = params.host.unwrap_or_default();
    if host.trim().is_empty() {
        return Err("missing 'host' parameter".into());
    }
    Ok(ValidParams {
        host,
        timeout: params.timeout.unwrap_or(5),
        probes: params.probes.unwrap_or(4),
        interval: params.interval.unwrap_or(1),
    })
}

pub async fn handler(
    State(_state): State<AppState>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let raw_params: Params =
        serde_urlencoded::from_str(query.as_deref().unwrap_or("")).unwrap_or_default();

    if query.as_deref().unwrap_or("").is_empty() {
        return ui::ping::page(&Params::default(), None, None);
    }

    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return ui::ping::page(&raw_params, None, Some(&msg));
        }
    };

    let timeout = Duration::from_secs(params.timeout);
    let interval = Duration::from_secs(params.interval);
    let result =
        ipgeom_query::ping(&params.host, timeout, params.probes, interval, None, None).await;
    match result {
        Ok(res) => ui::ping::page(&Params::from(&params), Some(&res), None),
        Err(e) => ui::ping::page(&Params::from(&params), None, Some(&e.to_string())),
    }
}
