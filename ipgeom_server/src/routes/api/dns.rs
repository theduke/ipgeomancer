use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::{routes::dns::parse_params, util, AppState};

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
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return util::json_error(axum::http::StatusCode::BAD_REQUEST, &msg).into_response()
        }
    };

    let server_ref = params.server.as_deref();
    match ipgeom_query::dns::authoritative_query(&params.name, params.record_type, server_ref).await
    {
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
        Err(e) => {
            util::json_error(axum::http::StatusCode::BAD_REQUEST, &e.to_string()).into_response()
        }
    }
}
