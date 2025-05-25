use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use hickory_proto::rr::RecordType;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

use crate::AppState;

#[derive(Deserialize)]
pub struct Params {
    name: String,
    record_type: Option<String>,
    server: Option<String>,
}

#[derive(Serialize)]
pub struct DnsRecord {
    name: String,
    ttl: u32,
    record_type: String,
    data: String,
}

#[derive(Serialize)]
pub struct DnsResponse {
    authoritative_server: String,
    records: Vec<DnsRecord>,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let rtype = params
        .record_type
        .as_deref()
        .and_then(|s| RecordType::from_str(s).ok())
        .unwrap_or(RecordType::A);
    let server = params.server.as_deref().filter(|s| !s.is_empty());
    match ipgeom_query::dns::authoritative_query(&params.name, rtype, server).await {
        Ok(res) => {
            let records = res
                .records
                .into_iter()
                .map(|r| DnsRecord {
                    name: r.name().to_utf8(),
                    ttl: r.ttl(),
                    record_type: r.record_type().to_string(),
                    data: r.data().to_string(),
                })
                .collect();
            Json(DnsResponse {
                authoritative_server: res.authoritative_server,
                records,
            })
            .into_response()
        }
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
