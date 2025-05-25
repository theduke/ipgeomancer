use axum::extract::State;
use axum::response::IntoResponse;

use crate::{AppState, ui};

pub async fn handler(State(_state): State<AppState>) -> impl IntoResponse {
    ui::home::page()
}
