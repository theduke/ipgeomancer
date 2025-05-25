use std::net::IpAddr;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{AppState, ui, util};

#[derive(Deserialize)]
pub struct Params {
    ip: Option<IpAddr>,
}

/// Lookup an arbitrary IP address submitted via a form.
pub async fn handler(
    State(state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    if let Some(ip) = params.ip {
        let countries = util::lookup_countries(state.db.as_ref(), ip).unwrap_or_default();
        let obj = util::lookup_inet_object(state.db.as_ref(), ip)
            .ok()
            .flatten();
        ui::lookup::page(
            Some(ip),
            Some(&countries),
            obj.as_ref().map(|(t, v)| (t.as_str(), v)),
        )
    } else {
        ui::lookup::page(None, None, None)
    }
}
