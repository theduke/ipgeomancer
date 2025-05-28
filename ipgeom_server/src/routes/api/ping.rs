use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::time::Duration;

use crate::{routes::ping::parse_params, util, AppState};

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
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return util::json_error(axum::http::StatusCode::BAD_REQUEST, &msg).into_response()
        }
    };

    let timeout = Duration::from_secs(params.timeout);
    let interval = Duration::from_secs(params.interval);
    match ipgeom_query::ping(&params.host, timeout, params.probes, interval, None, None).await {
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
        Err(e) => {
            util::json_error(axum::http::StatusCode::BAD_REQUEST, &e.to_string()).into_response()
        }
    }
}
