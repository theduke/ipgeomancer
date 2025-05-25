use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{AppState, ui};

#[derive(Deserialize)]
pub struct Params {
    domain: Option<String>,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    use ui::common::{layout, notification_error, notification_success, page_header};
    let intro = page_header("Domain Certificate", "Fetch TLS certificate information.");
    if let Some(domain) = params.domain.as_deref() {
        let result = ipgeom_query::fetch_certificate(domain).await;
        let body = match result {
            Ok(info) => maud::html! {
                (intro)
                (ui::cert::form(Some(domain)))
                (notification_success("Certificate fetched"))
                (ui::cert::result(&info))
            },
            Err(e) => maud::html! {
                (intro)
                (ui::cert::form(Some(domain)))
                (notification_error(&e.to_string()))
            },
        };
        layout("Domain Certificate", body)
    } else {
        let body = maud::html! { (intro) (ui::cert::form(None)) };
        layout("Domain Certificate", body)
    }
}
