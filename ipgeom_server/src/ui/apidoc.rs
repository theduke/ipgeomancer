use axum::response::Html;
use maud::html;

use super::common::{layout, page_header};

pub fn page() -> Html<String> {
    let body = html! {
        (page_header("API", "REST API Endpoints"))
        table class="table" {
            tr { th { "Endpoint" } th { "Query Parameters" } th { "Response" } }
            tr {
                td { code { "/api/v1/query/dns" } }
                td {
                    code { "name" } " (required)" br;
                    code { "record_type" } " (optional)" br;
                    code { "server" } " (optional)"
                }
                td { pre { "{\"authoritative_server\": String, \"records\": [{\"name\": String, \"ttl\": u32, \"record_type\": String, \"data\": String}]}" } }
            }
            tr {
                td { code { "/api/v1/query/whois" } }
                td { code { "domain" } " (required)" }
                td { pre { "{\"server\": \"whois.example.com\", \"data\": \"...\"}" } }
            }
            tr {
                td { code { "/api/v1/query/rdap" } }
                td {
                    code { "query" } " (required)" br;
                    code { "qtype" } " (optional)"
                }
                td { pre { "{\"objectClassName\": \"domain\", ...}" } }
            }
            tr {
                td { code { "/api/v1/ping" } }
                td {
                    code { "host" } " (required)" br;
                    code { "timeout" } " (optional)" br;
                    code { "probes" } " (optional)" br;
                    code { "interval" } " (optional)"
                }
                td { pre { "{\"ip\": \"1.2.3.4\", ...}" } }
            }
            tr {
                td { code { "/api/v1/query/traceroute" } }
                td {
                    code { "host" } " (required)" br;
                    code { "max_hops" } " (optional)" br;
                    code { "queries" } " (optional)" br;
                    code { "wait" } " (optional)"
                }
                td { pre { "{\"destination\": \"1.2.3.4\", \"hops\": [...] }" } }
            }
            tr {
                td { code { "/api/v1/query/domain-certificate" } }
                td { code { "domain" } " (required)" }
                td { pre { "{\"subject\": \"...\", \"issuer\": \"...\", \"not_before\": \"...\", \"not_after\": \"...\", \"valid\": true}" } }
            }
        }
    };
    layout("API", body)
}
