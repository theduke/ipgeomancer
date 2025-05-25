use axum::{extract::State, response::IntoResponse};

use crate::{ui, AppState};

/// Show REST API documentation.
pub async fn handler(State(_state): State<AppState>) -> impl IntoResponse {
    ui::apidoc::page()
}
