use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
};
use serde::Deserialize;
use std::str::FromStr;

use crate::{ui, AppState};

#[derive(Deserialize, Default)]
pub struct Params {
    pub query: Option<String>,
    pub qtype: Option<String>,
}

pub struct ValidParams {
    pub query_type: icann_rdap_client::rdap::QueryType,
    pub query: String,
    pub qtype: Option<String>,
}

impl From<&ValidParams> for Params {
    fn from(v: &ValidParams) -> Self {
        Self {
            query: Some(v.query.clone()),
            qtype: v.qtype.clone(),
        }
    }
}

pub fn parse_params(query: Option<&str>) -> Result<ValidParams, String> {
    let params: Params =
        serde_urlencoded::from_str(query.unwrap_or("")).map_err(|_| "invalid query parameters")?;
    let q = params.query.unwrap_or_default();
    if q.trim().is_empty() {
        return Err("missing 'query' parameter".into());
    }
    let qt = if let Some(t) = params.qtype.as_deref() {
        parse_query_type(t, &q).map_err(|e| e.to_string())?
    } else {
        icann_rdap_client::rdap::QueryType::from_str(&q).map_err(|e| e.to_string())?
    };
    Ok(ValidParams {
        query_type: qt,
        query: q,
        qtype: params.qtype,
    })
}

pub async fn handler(
    State(_state): State<AppState>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let raw_params: Params =
        serde_urlencoded::from_str(query.as_deref().unwrap_or("")).unwrap_or_default();

    if query.as_deref().unwrap_or("").is_empty() {
        return ui::rdap::page(&Params::default(), None, None);
    }

    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return ui::rdap::page(&raw_params, None, Some(&msg));
        }
    };

    let page_params = Params::from(&params);
    match ipgeom_query::rdap(params.query_type).await {
        Ok(res) => ui::rdap::page(&page_params, Some(&res), None),
        Err(e) => ui::rdap::page(&page_params, None, Some(&e.to_string())),
    }
}

fn parse_query_type(t: &str, value: &str) -> anyhow::Result<icann_rdap_client::rdap::QueryType> {
    use icann_rdap_client::rdap::QueryType;
    match t {
        "ipv4" => Ok(QueryType::ipv4(value)?),
        "ipv6" => Ok(QueryType::ipv6(value)?),
        "ipv4cidr" => Ok(QueryType::ipv4cidr(value)?),
        "ipv6cidr" => Ok(QueryType::ipv6cidr(value)?),
        "as" => Ok(QueryType::autnum(value)?),
        "domain" => Ok(QueryType::domain(value)?),
        "alabel" => Ok(QueryType::alabel(value)?),
        "ns" => Ok(QueryType::ns(value)?),
        "entity" => Ok(QueryType::Entity(value.to_string())),
        "domain-ns-ip" => Ok(QueryType::domain_ns_ip_search(value)?),
        "ns-ip" => Ok(QueryType::ns_ip_search(value)?),
        "url" => Ok(QueryType::Url(value.to_string())),
        "help" => Ok(QueryType::Help),
        _ => Err(anyhow::anyhow!("invalid query type")),
    }
}
