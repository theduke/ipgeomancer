use std::net::IpAddr;

use axum::response::Html;
use maud::{html, Markup};
use serde_json::Value;
use std::string::String;

use super::common::{hx_get_form, ip_info, layout, page_header, under_construction_warning};

pub fn form(ip: Option<IpAddr>) -> Markup {
    let value = ip.map(|i| i.to_string()).unwrap_or_default();
    let inner = html! {
        div class="field has-addons" {
            div class="control" {
                input class="input" type="text" name="ip" value=(value) placeholder="e.g. 1.1.1.1" required;
            }
            div class="control" { button type="submit" class="button is-primary" { "Lookup" } }
        }
    };
    hx_get_form("/lookup", inner)
}

pub fn inet_object_info(obj_type: &str, obj: &Value) -> Markup {
    let json = serde_json::to_string_pretty(obj).unwrap_or_default();
    html! { h3 { (obj_type) } pre { (json) } }
}

pub fn page(
    ip: Option<IpAddr>,
    countries: Option<&[String]>,
    obj: Option<(&str, &Value)>,
) -> Html<String> {
    let body = html! {
        (page_header("IP Lookup", "Look up the location of any IP address."))
        (under_construction_warning())
        (form(ip))
        @if let Some(addr) = ip {
            (ip_info(addr, countries.unwrap_or(&[])))
            @if let Some((t, v)) = obj {
                (inet_object_info(t, v))
            }
        }
    };
    layout("IP Lookup", "Look up the location of any IP address.", body)
}
