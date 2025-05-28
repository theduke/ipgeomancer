use axum::response::Html;
use maud::html;

use super::common::{layout, page_header};

pub fn page(path: &str) -> Html<String> {
    let body = html! {
        (page_header("Not Found", "The requested page could not be found."))
        p { "No page was found at \"" (path) "\"." }
    };
    layout("Not Found", body)
}
