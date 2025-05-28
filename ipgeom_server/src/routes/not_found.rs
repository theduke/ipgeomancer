use axum::{extract::State, http::StatusCode, http::Uri, response::IntoResponse};

use crate::{ui, AppState};

pub async fn handler(State(_state): State<AppState>, uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, ui::not_found::page(uri.path()))
}
