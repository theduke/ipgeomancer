use axum::extract::State;
use axum::response::IntoResponse;

use crate::{ui, AppState};

pub async fn handler(State(_state): State<AppState>) -> impl IntoResponse {
    ui::home::page()
}
