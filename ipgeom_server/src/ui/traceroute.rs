use maud::{html, Markup};

use super::common::hx_get_form;
use crate::routes::traceroute::Params;

pub fn form(params: &Params) -> Markup {
    let host_val = params.host.as_deref().unwrap_or("");
    let max_val = params.max_hops.unwrap_or(30);
    let queries_val = params.queries.unwrap_or(3);
    let wait_val = params.wait.unwrap_or(3);
    let inner = html! {
        div class="field is-grouped is-grouped-multiline" {
            div class="field" {
                label class="label" { "Host" }
                div class="control" { input class="input" type="text" name="host" value=(host_val) placeholder="example.com" required; }
            }
            div class="field" {
                label class="label" { "Max hops" }
                div class="control" { input class="input" type="number" name="max_hops" value=(max_val) min="1"; }
            }
            div class="field" {
                label class="label" { "Queries" }
                div class="control" { input class="input" type="number" name="queries" value=(queries_val) min="1"; }
            }
            div class="field" {
                label class="label" { "Wait" }
                div class="control" { input class="input" type="number" name="wait" value=(wait_val) min="1"; }
            }
            div class="field" { div class="control" { button type="submit" class="button is-primary" { "Trace" } } }
        }
    };
    hx_get_form("/traceroute", inner)
}

pub fn result(res: &ipgeom_query::TracerouteResult) -> Markup {
    fn hop_line(hop: &ipgeom_query::TracerouteHop) -> String {
        let mut out = String::new();
        let mut last_addr: Option<String> = None;
        let mut last_host: Option<String> = None;
        for p in &hop.probes {
            if let Some(rtt) = p.rtt_ms {
                let addr = p
                    .address
                    .map(|ip| ip.to_string())
                    .unwrap_or_else(|| "*".into());
                let host = p.hostname.clone().unwrap_or_else(|| addr.clone());
                if last_addr.as_deref() != Some(&addr) || last_host.as_deref() != Some(&host) {
                    out.push_str(&format!("{} ({}) ", host, addr));
                    last_addr = Some(addr.clone());
                    last_host = Some(host.clone());
                }
                out.push_str(&format!("{:.3} ms ", rtt));
            } else {
                out.push_str("* ");
            }
        }
        out
    }

    html! {
        p { "Destination: " (res.destination) }
        table class="table" {
            @for hop in &res.hops {
                tr {
                    td { (hop.ttl) }
                    td { (hop_line(hop)) }
                }
            }
        }
    }
}

pub fn page(
    params: &Params,
    result: Option<&ipgeom_query::TracerouteResult>,
    error: Option<&str>,
) -> axum::response::Html<String> {
    use super::common::{layout, notification_error, notification_success, page_header};
    let body = html! {
        (page_header("Traceroute", "Trace the network path to a host."))
        (form(params))
        @if let Some(err) = error { (notification_error(err)) }
        @if let Some(res) = result {
            (notification_success("Traceroute completed"))
            (self::result(res))
        }
    };
    layout("Traceroute", "Trace the network path to a host.", body)
}
