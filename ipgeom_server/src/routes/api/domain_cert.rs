use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::AppState;

#[derive(Deserialize)]
pub struct Params {
    domain: String,
}

#[derive(Serialize)]
pub struct CertResponse {
    subject: String,
    issuer: String,
    not_before: String,
    not_after: String,
    valid: bool,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    match ipgeom_query::fetch_certificate(&params.domain).await {
        Ok(info) => Json(CertResponse {
            subject: info.subject,
            issuer: info.issuer,
            not_before: info.not_before,
            not_after: info.not_after,
            valid: info.valid,
        })
        .into_response(),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
