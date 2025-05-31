use axum::response::Html;
use maud::{html, Markup};

use super::common::{layout, page_header};

fn endpoint(
    host: &str,
    route: &str,
    desc: &str,
    params: Markup,
    response: &str,
    example_path: &str,
) -> Markup {
    let url = format!("https://{}{}", host, example_path);
    html! {
        div class="box" {
            h3 class="title is-5" { code { (route) } }
            p { (desc) }
            p { b { "GET parameters:" } }
            (params)
            p { b { "Response:" } }
            pre { (response) }
            p { b { "curl:" } }
            pre { (format!("curl '{}'", url)) }
        }
    }
}

pub fn page(host: &str) -> Html<String> {
    let dns_params = html! {
        ul {
            li { code { "name" } " - domain name (required)" }
            li { code { "record_type" } " - DNS record type (optional)" }
            li { code { "server" } " - DNS server to query (optional)" }
        }
    };
    let whois_params = html! {
        ul { li { code { "domain" } " - domain name (required)" } }
    };
    let rdap_params = html! {
        ul {
            li { code { "query" } " - value to query (required)" }
            li { code { "qtype" } " - query type (optional)" }
        }
    };
    let ping_params = html! {
        ul {
            li { code { "host" } " - target host (required)" }
            li { code { "timeout" } " - seconds to wait (optional)" }
            li { code { "probes" } " - number of pings (optional)" }
            li { code { "interval" } " - interval seconds (optional)" }
        }
    };
    let trace_params = html! {
        ul {
            li { code { "host" } " - target host (required)" }
            li { code { "max_hops" } " - maximum hops (optional)" }
            li { code { "queries" } " - probes per hop (optional)" }
            li { code { "wait" } " - seconds to wait per probe (optional)" }
        }
    };
    let cert_params = html! {
        ul { li { code { "domain" } " - domain name (required)" } }
    };
    let hash_params = html! {
        ul {
            li { code { "method" } " - hashing method (only bcrypt supported)" }
            li { code { "password" } " - password to hash (required)" }
        }
    };

    let body = html! {
        (page_header("API", "REST API Endpoints"))
        div class="content" {
            (endpoint(
                host,
                "GET /api/v1/query/dns",
                "Query DNS records from the authoritative server.",
                dns_params,
                r#"{\"authoritative_server\": \"ns.example.com\", \"records\": [{\"name\": \"example.com.\", \"ttl\": 300, \"record_type\": \"A\", \"data\": \"93.184.216.34\"}]}"#,
                "/api/v1/query/dns?name=example.com&record_type=A",
            ))
            (endpoint(
                host,
                "GET /api/v1/query/whois",
                "Query WHOIS information for a domain.",
                whois_params,
                r#"{\"server\": \"whois.example.com\", \"data\": \"...\"}"#,
                "/api/v1/query/whois?domain=example.com",
            ))
            (endpoint(
                host,
                "GET /api/v1/query/rdap",
                "Perform an RDAP query (domains, IPs, ASNs, ...).",
                rdap_params,
                r#"{\"objectClassName\": \"domain\", ...}"#,
                "/api/v1/query/rdap?query=example.com",
            ))
            (endpoint(
                host,
                "GET /api/v1/ping",
                "Send ICMP echo requests to a host.",
                ping_params,
                r#"{\"ip\": \"1.2.3.4\", ...}"#,
                "/api/v1/ping?host=example.com&probes=1",
            ))
            (endpoint(
                host,
                "GET /api/v1/query/traceroute",
                "Trace the network path to a host.",
                trace_params,
                r#"{\"destination\": \"1.2.3.4\", \"hops\": [...]}"#,
                "/api/v1/query/traceroute?host=example.com&max_hops=1",
            ))
            (endpoint(
                host,
                "GET /api/v1/query/domain-certificate",
                "Fetch TLS certificate information for a domain.",
                cert_params,
                r#"{\"subject\": \"...\", \"issuer\": \"...\", \"not_before\": \"...\", \"not_after\": \"...\", \"valid\": true}"#,
                "/api/v1/query/domain-certificate?domain=example.com",
            ))
            (endpoint(
                host,
                "GET /api/v1/password-hash-generate",
                "Generate a password hash.",
                hash_params,
                r#"{\"method\": \"bcrypt\", \"hash\": \"$2b$...\"}"#,
                "/api/v1/password-hash-generate?method=bcrypt&password=secret",
            ))
        }
    };
    layout("API", "REST API Endpoints", body)
}
