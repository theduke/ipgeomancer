use maud::{html, Markup};

use super::common::hx_get_form;
use crate::routes::password_hash::Params;

pub fn form(params: &Params) -> Markup {
    let method_val = params.method.as_deref().unwrap_or("bcrypt");
    let password_val = params.password.as_deref().unwrap_or("");
    let inner = html! {
        div class="field is-grouped is-grouped-multiline" {
            div class="field" {
                label class="label" { "Method" }
                div class="control" {
                    div class="select" {
                        select name="method" {
                            option value="bcrypt" selected[method_val == "bcrypt"] { "bcrypt" }
                        }
                    }
                }
            }
            div class="field" {
                label class="label" { "Password" }
                div class="control" { input class="input" type="text" name="password" value=(password_val) required; }
            }
            div class="field" { div class="control" { button type="submit" class="button is-primary" { "Generate" } } }
        }
    };
    hx_get_form("/password-hash", inner)
}

pub fn result(hash: &str) -> Markup {
    html! { pre { (hash) } }
}

pub fn page(
    params: &Params,
    result: Option<&str>,
    error: Option<&str>,
) -> axum::response::Html<String> {
    use super::common::{layout, notification_error, page_header};
    let body = html! {
        (page_header("Password Hash", "Generate password hashes."))
        (form(params))
        @if let Some(err) = error { (notification_error(err)) }
        @if let Some(res) = result { (self::result(res)) }
    };
    layout(
        "Password Hash",
        "Generate a password hash for web servers",
        body,
    )
}
