use maud::{html, Markup};

use super::common::hx_get_form;
use crate::routes::cert::Params;

pub fn form(params: &Params) -> Markup {
    let domain_val = params.domain.as_deref().unwrap_or("");
    let inner = html! {
        div class="field has-addons" {
            div class="control" { input class="input" type="text" name="domain" value=(domain_val) placeholder="example.com" required; }
            div class="control" { button type="submit" class="button is-primary" { "Check" } }
        }
    };
    hx_get_form("/cert", inner)
}

pub fn result(info: &ipgeom_query::CertificateInfo) -> Markup {
    html! {
        table class="table" {
            tr { th { "Subject" } td { (info.subject) } }
            tr { th { "Issuer" } td { (info.issuer) } }
            tr { th { "Valid From" } td { (info.not_before) } }
            tr { th { "Valid To" } td { (info.not_after) } }
            tr { th { "Validation" } td { (if info.valid { "OK" } else { "FAILED" }) } }
        }
    }
}

pub fn page(
    params: &Params,
    result: Option<&ipgeom_query::CertificateInfo>,
    error: Option<&str>,
) -> axum::response::Html<String> {
    use super::common::{layout, notification_error, notification_success, page_header};
    let body = html! {
        (page_header("Domain Certificate", "Fetch TLS certificate information."))
        (form(params))
        @if let Some(err) = error { (notification_error(err)) }
        @if let Some(res) = result {
            (notification_success("Certificate fetched"))
            (self::result(res))
        }
    };
    layout(
        "Domain Certificate",
        "Fetch TLS certificate information.",
        body,
    )
}
