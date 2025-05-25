use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::AppState;

#[derive(Deserialize)]
pub struct Params {
    domain: String,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    match ipgeom_query::domain_whois(&params.domain).await {
        Ok(res) => Json(res).into_response(),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
