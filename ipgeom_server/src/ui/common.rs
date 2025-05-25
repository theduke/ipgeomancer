use axum::response::Html;
use maud::{DOCTYPE, Markup, html};
use std::net::IpAddr;

/// Wrap body HTML in a basic Bulma layout.
pub fn layout(title: &str, body: Markup) -> Html<String> {
    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { (title) }

                // Bulma CSS
                link
                    rel="stylesheet"
                    href="https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css" {}

                // htmx JS
                script src="https://unpkg.com/htmx.org@2.0.4/dist/htmx.min.js"
                {}
            }

            body {
                header class="hero is-primary" {
                    div class="hero-body" {
                        div class="container" {
                            h1 class="title" { "IpGeomancer" }
                        }
                    }
                }
                nav class="navbar is-light" {
                    div class="navbar-menu" {
                        div class="navbar-start" {
                            span class="navbar-item" { "Tools:" }
                            a class="navbar-item" href="/dns" { "DNS" }
                            a class="navbar-item" href="/whois" { "WHOIS" }
                            a class="navbar-item" href="/rdap" { "RDAP" }
                            a class="navbar-item" href="/ping" { "Ping" }
                            a class="navbar-item" href="/traceroute" { "Traceroute" }
                            a class="navbar-item" href="/cert" { "Domain Cert" }
                            a class="navbar-item" href="/api" { "API" }
                            a class="navbar-item" href="/myip" { "My IP" }
                            a class="navbar-item" href="/lookup" { "IP Lookup" }
                        }
                    }
                }
                main class="section" {
                    div class="container" { (body) }
                }
                footer class="footer" {
                    div class="content has-text-centered" {
                        "Powered by IpGeomancer tools - "
                        a href="https://github.com/theduke/ipgeomancer" { "github.com/theduke/ipgeomancer" }
                    }
                }
            }
        }
    };
    Html(markup.into_string())
}

/// Render a notification that the feature is still under construction.
pub fn under_construction_warning() -> Markup {
    html! { div class="notification is-warning" { "This functionality is under construction and may not work correctly." } }
}

/// Render a paragraph with IP information.
pub fn ip_info(ip: IpAddr, countries: &[String]) -> Markup {
    if countries.is_empty() {
        html! { p { "IP address: " (ip) " (country unknown)" } }
    } else if countries.len() == 1 {
        html! { p { "IP address: " (ip) " (country " (countries[0]) ")" } }
    } else {
        let list = countries.join(", ");
        html! { p { "IP address: " (ip) " (countries: " (list) ")" } }
    }
}

/// Render a page heading with a short description.
pub fn page_header(title: &str, desc: &str) -> Markup {
    html! {
        div class="block" {
            h2 class="title is-4" { (title) }
            p class="subtitle" { (desc) }
        }
    }
}

/// Render a success notification.
pub fn notification_success(msg: &str) -> Markup {
    html! { div class="notification is-success" { (msg) } }
}

/// Render an error notification.
pub fn notification_error(msg: &str) -> Markup {
    html! { div class="notification is-danger" { (msg) } }
}

/// Wrap the given controls in a form that submits via htmx.
///
/// The form uses the GET method and enables `hx-boost` so that the entire page
/// is replaced with the server response. While the request is in flight the
/// submit button receives Bulma's `is-loading` class to show a spinner.
pub fn hx_get_form(action: &str, controls: Markup) -> Markup {
    html! {
        form
            action=(action)
            method="get"
            hx-boost="true"
            hx-get=(action)
            hx-target="body"
            hx-on:submit="this.querySelector('button[type=\"submit\"]').classList.add('is-loading');"
        {
            (controls)
        }
    }
}
