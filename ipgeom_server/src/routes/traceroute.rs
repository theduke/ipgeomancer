use async_traceroute::ProbeMethod;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use std::time::Duration;

use crate::{AppState, ui};

#[derive(Deserialize)]
pub struct Params {
    host: Option<String>,
    max_hops: Option<u8>,
    queries: Option<u16>,
    wait: Option<u64>,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    use ui::common::{layout, notification_error, notification_success, page_header};
    let intro = page_header("Traceroute", "Trace the network path to a host.");
    if let Some(host) = params.host.as_deref() {
        let max_hops = params.max_hops.unwrap_or(30);
        let queries = params.queries.unwrap_or(3);
        let wait = Duration::from_secs(params.wait.unwrap_or(3));
        let host_string = host.to_string();
        let handle = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            handle.block_on(ipgeom_query::traceroute(
                &host_string,
                ProbeMethod::UDP,
                max_hops,
                queries,
                wait,
                16,
                None,
                true,
                None,
                None,
            ))
        })
        .await
        .unwrap();
        let body = match result {
            Ok(res) => maud::html! {
                (intro)
                (ui::traceroute::form(Some(host), Some(max_hops), Some(queries), Some(wait.as_secs())))
                (notification_success("Traceroute completed"))
                (ui::traceroute::result(&res))
            },
            Err(e) => maud::html! {
                (intro)
                (ui::traceroute::form(Some(host), Some(max_hops), Some(queries), Some(wait.as_secs())))
                (notification_error(&e.to_string()))
            },
        };
        layout("Traceroute", body)
    } else {
        let body = maud::html! { (intro) (ui::traceroute::form(None, None, None, None)) };
        layout("Traceroute", body)
    }
}
