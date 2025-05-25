use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use icann_rdap_client::rdap::QueryType;
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;

use crate::AppState;

#[derive(Deserialize)]
pub struct Params {
    query: String,
    qtype: Option<String>,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let qt_res = if let Some(t) = params.qtype.as_deref() {
        parse_query_type(t, &params.query)
    } else {
        QueryType::from_str(&params.query).map_err(|e| e.into())
    };

    match qt_res {
        Ok(qt) => match ipgeom_query::rdap(qt).await {
            Ok(res) => Json(res).into_response(),
            Err(e) => (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        },
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

fn parse_query_type(t: &str, value: &str) -> anyhow::Result<QueryType> {
    match t {
        "ipv4" => Ok(QueryType::ipv4(value)?),
        "ipv6" => Ok(QueryType::ipv6(value)?),
        "ipv4cidr" => Ok(QueryType::ipv4cidr(value)?),
        "ipv6cidr" => Ok(QueryType::ipv6cidr(value)?),
        "as" => Ok(QueryType::autnum(value)?),
        "domain" => Ok(QueryType::domain(value)?),
        "alabel" => Ok(QueryType::alabel(value)?),
        "ns" => Ok(QueryType::ns(value)?),
        "entity" => Ok(QueryType::Entity(value.to_string())),
        "domain-ns-ip" => Ok(QueryType::domain_ns_ip_search(value)?),
        "ns-ip" => Ok(QueryType::ns_ip_search(value)?),
        "url" => Ok(QueryType::Url(value.to_string())),
        "help" => Ok(QueryType::Help),
        _ => Err(anyhow::anyhow!("invalid query type")),
    }
}
