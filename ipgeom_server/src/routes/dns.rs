use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use hickory_proto::rr::RecordType;
use serde::Deserialize;
use std::str::FromStr;

use crate::{ui, AppState};

#[derive(Deserialize)]
pub struct Params {
    name: Option<String>,
    record_type: Option<String>,
    server: Option<String>,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    use ui::common::{layout, notification_error, notification_success, page_header};
    let intro = page_header(
        "DNS Query",
        "Query DNS records against the authoritative server.",
    );
    if let (Some(name), Some(record_type)) = (params.name.as_deref(), params.record_type.as_deref())
    {
        let rtype = RecordType::from_str(record_type).unwrap_or(RecordType::A);
        let server = params
            .server
            .as_deref()
            .and_then(|s| if s.is_empty() { None } else { Some(s) });
        let result = ipgeom_query::dns::authoritative_query(name, rtype, server).await;
        let body = match result {
            Ok(res) => maud::html! {
                (intro)
                (ui::dns::form(Some(name), Some(record_type), params.server.as_deref()))
                (notification_success("Query successful"))
                (ui::dns::records(&res.authoritative_server, &res.records))
            },
            Err(e) => maud::html! {
                (intro)
                (ui::dns::form(Some(name), Some(record_type), params.server.as_deref()))
                (notification_error(&e.to_string()))
            },
        };
        layout("DNS Query", body)
    } else {
        let body = maud::html! { (intro) (ui::dns::form(None, None, None)) };
        layout("DNS Query", body)
    }
}
