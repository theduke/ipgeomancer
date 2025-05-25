use maud::{html, Markup};

use super::common::hx_get_form;

pub fn form(domain: Option<&str>) -> Markup {
    let domain_val = domain.unwrap_or("");
    let inner = html! {
        div class="field has-addons" {
            div class="control" { input class="input" type="text" name="domain" value=(domain_val) placeholder="example.com"; }
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
