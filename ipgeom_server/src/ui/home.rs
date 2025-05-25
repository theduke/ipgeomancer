use axum::response::Html;
use maud::{html, Markup};

use super::common::layout;

fn tool_link(url: &str, title: &str, desc: &str) -> Markup {
    html! {
        div class="column is-half" {
            div class="box" {
                h3 class="title is-5" { a href=(url) { (title) } }
                p { (desc) }
            }
        }
    }
}

pub fn page() -> Html<String> {
    let body = html! {
        div class="block" {
            h2 class="title is-4" { "Welcome to IpGeomancer" }
            p { "A set of tools to investigate internet infrastructure." }
        }
        div class="columns is-multiline" {
            (tool_link("/dns", "DNS", "Query DNS records against the authoritative server."))
            (tool_link("/whois", "WHOIS", "Query WHOIS information for a domain."))
            (tool_link("/rdap", "RDAP", "Retrieve RDAP information about domains and IPs."))
            (tool_link("/ping", "Ping", "Send ICMP echo requests to a host."))
            (tool_link("/traceroute", "Traceroute", "Trace the network path to a host."))
            (tool_link("/cert", "Domain Cert", "Fetch TLS certificate information."))
            (tool_link("/myip", "My IP", "Information about your current IP address."))
            (tool_link("/lookup", "IP Lookup", "Look up the location of any IP address."))
        }

        hr {}

        div {
            (tool_link("/api", "API", "REST API documentation."))
        }
    };
    layout("IpGeomancer", body)
}
