use axum::{extract::State, response::IntoResponse};

use crate::{AppState, ui};

/// Show REST API documentation.
pub async fn handler(State(_state): State<AppState>) -> impl IntoResponse {
    ui::apidoc::page()
}
