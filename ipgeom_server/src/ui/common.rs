use axum::response::Html;
use maud::{html, Markup, DOCTYPE};
use std::net::IpAddr;

/// Wrap body HTML in a basic Bulma layout.
///
/// The page title and a short description are used to populate meta
/// tags for SEO and social media previews.
pub fn layout(title: &str, description: &str, body: Markup) -> Html<String> {
    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="description" content=(description);
                meta property="og:title" content=(title);
                meta property="og:description" content=(description);
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
                header class="hero is-primary is-small" {
                    div class="hero-body" {
                        div class="container is-flex is-justify-content-space-between is-align-items-center" {
                            h1 class="title" {
                                a href="/" class="has-text-black" {
                                    "IpGeomancer"
                                }
                            }
                            a
                                class="icon is-large has-text-white"
                                href="https://github.com/theduke/ipgeomancer"
                                aria-label="GitHub repository"
                            {
                                svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24"
                                    width="40"
                                    height="40"
                                {
                                    path
                                        fill-rule="evenodd"
                                        d="M12 0a12 12 0 00-3.79 23.4c.6.11.82-.26.82-.58v-2.04c-3.34.73-4.04-1.61-4.04-1.61-.55-1.39-1.34-1.75-1.34-1.75-1.1-.75.08-.73.08-.73 1.2.09 1.84 1.24 1.84 1.24 1.07 1.83 2.8 1.3 3.49.99.11-.77.42-1.3.76-1.6-2.67-.3-5.47-1.33-5.47-5.93 0-1.31.47-2.38 1.24-3.22-.12-.3-.53-1.52.12-3.17 0 0 1-.32 3.3 1.23a11.6 11.6 0 016 0c2.3-1.55 3.3-1.23 3.3-1.23.65 1.65.25 2.87.12 3.17.77.84 1.24 1.91 1.24 3.22 0 4.61-2.8 5.62-5.48 5.92.43.37.82 1.1.82 2.22v3.29c0 .32.22.7.82.58A12 12 0 0012 0z"
                                    {}
                                }
                            }
                        }
                    }
                }
                nav class="navbar is-light" {
                    // Keep navbar items visible on mobile as well.
                    div class="navbar-menu is-active" {
                        div class="navbar-start" {
                            span class="navbar-item" { "Tools:" }
                            a class="navbar-item" href="/dns" { "DNS" }
                            a class="navbar-item" href="/whois" { "WHOIS" }
                            a class="navbar-item" href="/rdap" { "RDAP" }
                            a class="navbar-item" href="/ping" { "Ping" }
                            a class="navbar-item" href="/traceroute" { "Traceroute" }
                            a class="navbar-item" href="/cert" { "Domain Cert" }
                            a class="navbar-item" href="/myip" { "My IP" }
                            a class="navbar-item" href="/lookup" { "IP Lookup" }
                            hr class="navbar-divider" {}
                            a class="navbar-item" href="/api-docs" { "API" }
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
            class="mb-4"
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
