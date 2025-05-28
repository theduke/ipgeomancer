use maud::{html, Markup};

use super::common::hx_get_form;
use crate::routes::rdap::Params;

pub fn form(params: &Params) -> Markup {
    let query_val = params.query.as_deref().unwrap_or("");
    let qtype_val = params.qtype.as_deref().unwrap_or("domain");
    let types = [
        ("domain", "Domain"),
        ("ipv4", "IPv4"),
        ("ipv6", "IPv6"),
        ("ipv4cidr", "IPv4 CIDR"),
        ("ipv6cidr", "IPv6 CIDR"),
        ("as", "AS Number"),
        ("ns", "Nameserver"),
        ("alabel", "A-Label"),
        ("entity", "Entity"),
        ("domain-ns-ip", "Domain NS IP Search"),
        ("ns-ip", "Nameserver IP Search"),
        ("url", "URL"),
        ("help", "Help"),
    ];
    let inner = html! {
        div class="field is-grouped is-grouped-multiline" {
            div class="field" {
                label class="label" { "Query" }
                div class="control" { input class="input" type="text" name="query" value=(query_val) placeholder="query" required; }
            }
            div class="field" {
                label class="label" { "Type" }
                div class="control" {
                    select class="select" name="qtype" {
                        @for (val, label) in &types {
                            @if *val == qtype_val { option value=(val) selected { (label) } } @else { option value=(val) { (label) } }
                        }
                    }
                }
            }
            div class="field" { div class="control" { button type="submit" class="button is-primary" { "Query" } } }
        }
    };
    hx_get_form("/rdap", inner)
}

pub fn result(res: &icann_rdap_client::rdap::ResponseData) -> Markup {
    let json = serde_json::to_string_pretty(res).unwrap_or_default();
    html! { pre { (json) } }
}

pub fn page(
    params: &Params,
    result: Option<&icann_rdap_client::rdap::ResponseData>,
    error: Option<&str>,
) -> axum::response::Html<String> {
    use super::common::{layout, notification_error, notification_success, page_header};
    let body = html! {
        (page_header(
            "RDAP Lookup",
            "Retrieve RDAP information about domains and IPs.",
        ))
        (form(params))
        @if let Some(err) = error { (notification_error(err)) }
        @if let Some(res) = result {
            (notification_success("Lookup successful"))
            (self::result(res))
        }
    };
    layout("RDAP Lookup", body)
}
