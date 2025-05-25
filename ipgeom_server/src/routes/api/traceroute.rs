use async_traceroute::ProbeMethod;
use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use ipgeom_query::TracerouteHop;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

use crate::AppState;

#[derive(Deserialize)]
pub struct Params {
    host: String,
    max_hops: Option<u8>,
    queries: Option<u16>,
    wait: Option<u64>,
}

#[derive(Serialize)]
pub struct TracerouteResponse {
    destination: std::net::IpAddr,
    hops: Vec<TracerouteHop>,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let max_hops = params.max_hops.unwrap_or(30);
    let queries = params.queries.unwrap_or(3);
    let wait = Duration::from_secs(params.wait.unwrap_or(3));
    let host = params.host.clone();
    let handle = tokio::runtime::Handle::current();
    let result = tokio::task::spawn_blocking(move || {
        handle.block_on(ipgeom_query::traceroute::traceroute(
            &host,
            ProbeMethod::UDP,
            max_hops,
            queries,
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
        Ok(res) => Json(TracerouteResponse {
            destination: res.destination,
            hops: res.hops,
        })
        .into_response(),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
