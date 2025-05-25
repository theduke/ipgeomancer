use maud::{html, Markup};

use super::common::hx_get_form;

pub fn form(
    host: Option<&str>,
    timeout: Option<u64>,
    probes: Option<u16>,
    interval: Option<u64>,
) -> Markup {
    let host_val = host.unwrap_or("");
    let timeout_val = timeout.unwrap_or(5);
    let probes_val = probes.unwrap_or(4);
    let interval_val = interval.unwrap_or(1);
    let inner = html! {
        div class="field is-grouped is-grouped-multiline" {
            div class="field" {
                label class="label" { "Host" }
                div class="control" { input class="input" type="text" name="host" value=(host_val) placeholder="example.com"; }
            }
            div class="field" {
                label class="label" { "Timeout" }
                div class="control" { input class="input" type="number" name="timeout" value=(timeout_val) min="1"; }
            }
            div class="field" {
                label class="label" { "Probes" }
                div class="control" { input class="input" type="number" name="probes" value=(probes_val) min="1"; }
            }
            div class="field" {
                label class="label" { "Interval" }
                div class="control" { input class="input" type="number" name="interval" value=(interval_val) min="1"; }
            }
            div class="field" { div class="control" { button type="submit" class="button is-primary" { "Ping" } } }
        }
    };
    hx_get_form("/ping", inner)
}

pub fn result(res: &ipgeom_query::PingResult) -> Markup {
    let avg = res
        .avg_time_ms
        .map(|v| format!("{v:.2}"))
        .unwrap_or_else(|| "n/a".into());
    let min = res
        .min_time_ms
        .map(|v| format!("{v:.2}"))
        .unwrap_or_else(|| "n/a".into());
    let max = res
        .max_time_ms
        .map(|v| format!("{v:.2}"))
        .unwrap_or_else(|| "n/a".into());
    let mdev = res
        .stddev_ms
        .map(|v| format!("{v:.2}"))
        .unwrap_or_else(|| "n/a".into());
    html! {
        p { "IP: " (res.ip) }
        table class="table" {
            tr { th { "Seq" } th { "Source" } th { "Size" } th { "TTL" } th { "RTT" } }
            @for p in &res.pings {
                @if let Some(rtt) = p.rtt_ms {
                    tr {
                        td { (p.seq + 1) }
                        td { (p.source.map(|ip| ip.to_string()).unwrap_or_else(|| "-".into())) }
                        td { (p.size.map(|s| s.to_string()).unwrap_or_else(|| "-".into())) }
                        td { (p.ttl.map(|t| t.to_string()).unwrap_or_else(|| "-".into())) }
                        td { (format!("{:.2} ms", rtt)) }
                    }
                } @else {
                    tr { td { (p.seq + 1) } td colspan="4" { "timeout" } }
                }
            }
        }
        div class="notification is-success" {
            (res.transmitted) " packets transmitted, " (res.received) " received, min/avg/max/mdev = "
            (min) "/" (avg) "/" (max) "/" (mdev) " ms"
        }
    }
}
