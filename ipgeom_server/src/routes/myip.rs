use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    response::IntoResponse,
};

use crate::{AppState, ui, util};

/// Display information about the client's IP address.
pub async fn handler(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let ip = addr.ip();
    let countries = util::lookup_countries(state.db.as_ref(), ip).unwrap_or_default();
    ui::myip::page(ip, &countries)
}
