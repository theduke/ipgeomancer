use async_traceroute::ProbeMethod;
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
    pub max_hops: Option<u8>,
    pub queries: Option<u16>,
    pub wait: Option<u64>,
}

pub struct ValidParams {
    pub host: String,
    pub max_hops: u8,
    pub queries: u16,
    pub wait: u64,
}

impl From<&ValidParams> for Params {
    fn from(v: &ValidParams) -> Self {
        Self {
            host: Some(v.host.clone()),
            max_hops: Some(v.max_hops),
            queries: Some(v.queries),
            wait: Some(v.wait),
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
        max_hops: params.max_hops.unwrap_or(30),
        queries: params.queries.unwrap_or(3),
        wait: params.wait.unwrap_or(3),
    })
}

pub async fn handler(
    State(_state): State<AppState>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let raw_params: Params =
        serde_urlencoded::from_str(query.as_deref().unwrap_or("")).unwrap_or_default();

    if query.as_deref().unwrap_or("").is_empty() {
        return ui::traceroute::page(&Params::default(), None, None);
    }

    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return ui::traceroute::page(&raw_params, None, Some(&msg));
        }
    };

    let wait = Duration::from_secs(params.wait);
    let host_string = params.host.clone();
    let handle = tokio::runtime::Handle::current();
    let result = tokio::task::spawn_blocking(move || {
        handle.block_on(ipgeom_query::traceroute(
            &host_string,
            ProbeMethod::UDP,
            params.max_hops,
            params.queries,
            wait,
            16,
            None,
            true,
            None,
            None,
        ))
    })
    .await
    .unwrap();
    match result {
        Ok(res) => ui::traceroute::page(&Params::from(&params), Some(&res), None),
        Err(e) => ui::traceroute::page(&Params::from(&params), None, Some(&e.to_string())),
    }
}
