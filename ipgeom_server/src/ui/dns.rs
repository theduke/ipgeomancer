use hickory_proto::rr::Record;
use maud::{Markup, html};

use super::common::hx_get_form;

pub fn form(name: Option<&str>, record_type: Option<&str>, server: Option<&str>) -> Markup {
    let name_val = name.unwrap_or("");
    let server_val = server.unwrap_or("");
    let record_val = record_type.unwrap_or("A");
    let record_types = ["A", "AAAA", "MX", "NS", "CNAME", "TXT"];
    let inner = html! {
        div class="field is-grouped is-grouped-multiline" {
            div class="field" {
                label class="label" { "Name" }
                div class="control" {
                    input class="input" type="text" name="name" value=(name_val) placeholder="example.com";
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
