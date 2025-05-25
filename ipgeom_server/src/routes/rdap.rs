use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use std::str::FromStr;

use crate::{ui, AppState};

#[derive(Deserialize)]
pub struct Params {
    query: Option<String>,
    qtype: Option<String>,
}

pub async fn handler(
    State(_state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    use ui::common::{layout, notification_error, notification_success, page_header};
    let intro = page_header(
        "RDAP Lookup",
        "Retrieve RDAP information about domains and IPs.",
    );
    if let Some(query) = params.query.as_deref() {
        let qt_res = if let Some(t) = params.qtype.as_deref() {
            parse_query_type(t, query)
        } else {
            icann_rdap_client::rdap::QueryType::from_str(query).map_err(|e| e.into())
        };
        let body = match qt_res {
            Ok(qt) => match ipgeom_query::rdap(qt).await {
                Ok(res) => maud::html! {
                    (intro)
                    (ui::rdap::form(Some(query), params.qtype.as_deref()))
                    (notification_success("Lookup successful"))
                    (ui::rdap::result(&res))
                },
                Err(e) => maud::html! {
                    (intro)
                    (ui::rdap::form(Some(query), params.qtype.as_deref()))
                    (notification_error(&e.to_string()))
                },
            },
            Err(e) => maud::html! {
                (intro)
                (ui::rdap::form(Some(query), params.qtype.as_deref()))
                (notification_error(&e.to_string()))
            },
        };
        layout("RDAP Lookup", body)
    } else {
        let body = maud::html! { (intro) (ui::rdap::form(None, None)) };
        layout("RDAP Lookup", body)
    }
}

fn parse_query_type(t: &str, value: &str) -> anyhow::Result<icann_rdap_client::rdap::QueryType> {
    use icann_rdap_client::rdap::QueryType;
    match t {
        "ipv4" => Ok(QueryType::ipv4(value)?),
        "ipv6" => Ok(QueryType::ipv6(value)?),
        "ipv4cidr" => Ok(QueryType::ipv4cidr(value)?),
        "ipv6cidr" => Ok(QueryType::ipv6cidr(value)?),
        "as" => Ok(QueryType::autnum(value)?),
        "domain" => Ok(QueryType::domain(value)?),
        "alabel" => Ok(QueryType::alabel(value)?),
        "ns" => Ok(QueryType::ns(value)?),
        "entity" => Ok(QueryType::Entity(value.to_string())),
        "domain-ns-ip" => Ok(QueryType::domain_ns_ip_search(value)?),
        "ns-ip" => Ok(QueryType::ns_ip_search(value)?),
        "url" => Ok(QueryType::Url(value.to_string())),
        "help" => Ok(QueryType::Help),
        _ => Err(anyhow::anyhow!("invalid query type")),
    }
}
