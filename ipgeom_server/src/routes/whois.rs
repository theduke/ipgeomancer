use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{ui, AppState};

#[derive(Deserialize)]
pub struct Params {
    domain: Option<String>,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    use ui::common::{layout, notification_error, notification_success, page_header};
    let intro = page_header("WHOIS Lookup", "Query WHOIS information for a domain.");
    if let Some(domain) = params.domain.as_deref() {
        let result = ipgeom_query::domain_whois(domain).await;
        let body = match result {
            Ok(json) => maud::html! {
                (intro)
                (ui::whois::form(Some(domain)))
                (notification_success("Lookup successful"))
                (ui::whois::result(&json))
            },
            Err(e) => maud::html! {
                (intro)
                (ui::whois::form(Some(domain)))
                (notification_error(&e.to_string()))
            },
        };
        layout("WHOIS Lookup", body)
    } else {
        let body = maud::html! { (intro) (ui::whois::form(None)) };
        layout("WHOIS Lookup", body)
    }
}
