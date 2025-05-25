use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

use crate::AppState;

#[derive(Deserialize)]
pub struct Params {
    host: String,
    timeout: Option<u64>,
    probes: Option<u16>,
    interval: Option<u64>,
}

#[derive(Serialize)]
pub struct PingResponse {
    ip: std::net::IpAddr,
    transmitted: u16,
    received: u16,
    pings: Vec<ipgeom_query::PingUpdate>,
    avg_time_ms: Option<f64>,
    min_time_ms: Option<f64>,
    max_time_ms: Option<f64>,
    stddev_ms: Option<f64>,
    total_time_ms: f64,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let timeout = Duration::from_secs(params.timeout.unwrap_or(5));
    let probes = params.probes.unwrap_or(4);
    let interval = Duration::from_secs(params.interval.unwrap_or(1));
    match ipgeom_query::ping(&params.host, timeout, probes, interval, None, None).await {
        Ok(res) => Json(PingResponse {
            ip: res.ip,
            transmitted: res.transmitted,
            received: res.received,
            pings: res.pings,
            avg_time_ms: res.avg_time_ms,
            min_time_ms: res.min_time_ms,
            max_time_ms: res.max_time_ms,
            stddev_ms: res.stddev_ms,
            total_time_ms: res.total_time_ms,
        })
        .into_response(),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
