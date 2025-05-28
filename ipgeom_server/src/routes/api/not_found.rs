use axum::{extract::State, http::StatusCode, http::Uri, response::IntoResponse, Json};
use serde_json::json;

use crate::AppState;

pub async fn handler(State(_state): State<AppState>, _uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json(json!({"error": "Not Found"})))
}
