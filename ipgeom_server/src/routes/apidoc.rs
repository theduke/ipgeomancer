use axum::{
    extract::{OriginalUri, State},
    http::HeaderMap,
    response::IntoResponse,
};

use crate::{ui, AppState};

/// Show REST API documentation.
pub async fn handler(
    OriginalUri(uri): OriginalUri,
    State(_state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let host = headers
        .get(axum::http::header::HOST)
        .and_then(|h| h.to_str().ok().and_then(|s| s.split(':').next()))
        .or_else(|| uri.host())
        .unwrap_or("localhost");

    ui::apidoc::page(host)
}
