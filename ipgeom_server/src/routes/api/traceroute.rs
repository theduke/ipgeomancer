use async_traceroute::ProbeMethod;
use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
    Json,
};
use ipgeom_query::TracerouteHop;
use serde::Serialize;
use std::time::Duration;

use crate::{routes::traceroute::parse_params, util, AppState};

#[derive(Serialize)]
pub struct TracerouteResponse {
    destination: std::net::IpAddr,
    hops: Vec<TracerouteHop>,
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

    let wait = Duration::from_secs(params.wait);
    let host = params.host.clone();
    let handle = tokio::runtime::Handle::current();
    let result = tokio::task::spawn_blocking(move || {
        handle.block_on(ipgeom_query::traceroute::traceroute(
            &host,
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
        Ok(res) => Json(TracerouteResponse {
            destination: res.destination,
            hops: res.hops,
        })
        .into_response(),
        Err(e) => {
            util::json_error(axum::http::StatusCode::BAD_REQUEST, &e.to_string()).into_response()
        }
    }
}
