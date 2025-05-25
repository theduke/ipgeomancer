use std::net::IpAddr;

use axum::response::Html;
use maud::html;
use std::string::String;

use super::common::{ip_info, layout, page_header, under_construction_warning};

pub fn page(ip: IpAddr, countries: &[String]) -> Html<String> {
    let body = html! {
        (page_header("Your IP", "Information about your current IP address."))
        (under_construction_warning())
        (ip_info(ip, countries))
    };
    layout("Your IP", body)
}
