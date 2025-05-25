use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use std::time::Duration;

use crate::{ui, AppState};

#[derive(Deserialize)]
pub struct Params {
    host: Option<String>,
    timeout: Option<u64>,
    probes: Option<u16>,
    interval: Option<u64>,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    use ui::common::{layout, notification_error, page_header};
    let intro = page_header("Ping", "Send ICMP echo requests to a host.");
    if let Some(host) = params.host.as_deref() {
        let timeout = Duration::from_secs(params.timeout.unwrap_or(5));
        let probes = params.probes.unwrap_or(4);
        let interval = Duration::from_secs(params.interval.unwrap_or(1));
        let result = ipgeom_query::ping(host, timeout, probes, interval, None, None).await;
        let body = match result {
            Ok(res) => maud::html! {
                (intro)
                (ui::ping::form(Some(host), Some(timeout.as_secs()), Some(probes), Some(interval.as_secs())))
                (ui::ping::result(&res))
            },
            Err(e) => maud::html! {
                (intro)
                (ui::ping::form(Some(host), Some(timeout.as_secs()), Some(probes), Some(interval.as_secs())))
                (notification_error(&e.to_string()))
            },
        };
        layout("Ping", body)
    } else {
        let body = maud::html! { (intro) (ui::ping::form(None, None, None, None)) };
        layout("Ping", body)
    }
}
