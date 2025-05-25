use maud::{html, Markup};

use super::common::hx_get_form;

pub fn form(query: Option<&str>, qtype: Option<&str>) -> Markup {
    let query_val = query.unwrap_or("");
    let qtype_val = qtype.unwrap_or("domain");
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
                div class="control" { input class="input" type="text" name="query" value=(query_val) placeholder="query"; }
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
