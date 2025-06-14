use hickory_proto::rr::Record;
use maud::{html, Markup};

use super::common::hx_get_form;
use crate::routes::dns::Params;

pub fn form(params: &Params) -> Markup {
    let name_val = params.name.as_deref().unwrap_or("");
    let server_val = params.server.as_deref().unwrap_or("");
    let record_val = params.record_type.as_deref().unwrap_or("A");
    let record_types = ["A", "AAAA", "MX", "NS", "CNAME", "TXT"];
    let inner = html! {
        div class="field is-grouped is-grouped-multiline" {
            div class="field" {
                label class="label" { "Name" }
                div class="control" {
                    input class="input" type="text" name="name" value=(name_val) placeholder="example.com" required;
                }
            }
            div class="field" {
                label class="label" { "Type" }
                div class="control" {
                    select class="select" name="record_type" {
                        @for t in record_types {
                            @if t == record_val { option selected { (t) } } @else { option { (t) } }
                        }
                    }
                }
            }
            div class="field" {
                label class="label" { "Server" }
                div class="control" {
                    input class="input" type="text" name="server" value=(server_val) placeholder="optional server";
                }
            }
            div class="field" {
                div class="control" { button type="submit" class="button is-primary" { "Query" } }
            }
        }
    };
    hx_get_form("/dns", inner)
}

pub fn records(auth_server: &str, records: &[Record]) -> Markup {
    if records.is_empty() {
        html! {
            div class="notification is-danger" { "No records found." }
            p { "Authoritative server: " (auth_server) }
        }
    } else {
        html! {
            div class="notification is-success" { "Found " (records.len()) " record(s)." }
            p { "Authoritative server: " (auth_server) }
            table class="table" {
                tr { th { "Name" } th { "TTL" } th { "Type" } th { "Data" } }
                @for r in records {
                    tr {
                        td { (r.name()) }
                        td { (r.ttl()) }
                        td { (r.record_type()) }
                        td { (r.data()) }
                    }
                }
            }
        }
    }
}

pub fn page(
    params: &Params,
    records: Option<(&str, &[Record])>,
    error: Option<&str>,
) -> axum::response::Html<String> {
    use super::common::{layout, notification_error, page_header};
    let body = html! {
        (page_header(
            "DNS Query",
            "Query DNS records against the authoritative server.",
        ))
        (form(params))
        @if let Some(err) = error { (notification_error(err)) }
        @if let Some((auth, rec)) = records { (self::records(auth, rec)) }
    };
    layout(
        "DNS Query",
        "Query DNS records against the authoritative server.",
        body,
    )
}
