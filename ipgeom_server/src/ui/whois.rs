use maud::{html, Markup};

use super::common::hx_get_form;
use crate::routes::whois::Params;

pub fn form(params: &Params) -> Markup {
    let domain_val = params.domain.as_deref().unwrap_or("");
    let inner = html! {
        div class="field has-addons" {
            div class="control" { input class="input" type="text" name="domain" value=(domain_val) placeholder="example.com" required; }
            div class="control" { button class="button is-primary" type="submit" { "Lookup"}  }
        }
    };
    hx_get_form("/whois", inner)
}

pub fn result(res: &ipgeom_whois::WhoisResponse) -> Markup {
    let table = if let Some(fields) = res.parse() {
        if !fields.is_empty() {
            Some(html! {
                table class="table" {
                    tr { th { "Field" } th { "Value" } }
                    @for (k, v) in fields { tr { td { (k) } td { (v) } } }
                }
            })
        } else {
            None
        }
    } else {
        None
    };
    html! {
        @if let Some(t) = table { (t) }
        div style="margin-top: 2rem" {
            p { b { "Raw response" } }
            pre { (res.data) }
        }
    }
}

pub fn page(
    params: &Params,
    result: Option<&ipgeom_whois::WhoisResponse>,
    error: Option<&str>,
) -> axum::response::Html<String> {
    use super::common::{layout, notification_error, notification_success, page_header};
    let body = html! {
        (page_header("WHOIS Lookup", "Query WHOIS information for a domain."))
        (form(params))
        @if let Some(err) = error { (notification_error(err)) }
        @if let Some(res) = result {
            (notification_success("Lookup successful"))
            (self::result(res))
        }
    };
    layout(
        "WHOIS Lookup",
        "Query WHOIS information for a domain.",
        body,
    )
}
