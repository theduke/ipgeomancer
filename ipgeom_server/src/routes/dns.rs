use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
};
use hickory_proto::rr::RecordType;
use serde::Deserialize;
use std::str::FromStr;

use crate::{ui, AppState};

#[derive(Deserialize, Default)]
pub struct Params {
    pub name: Option<String>,
    pub record_type: Option<String>,
    pub server: Option<String>,
}

pub struct ValidParams {
    pub name: String,
    pub record_type: RecordType,
    pub server: Option<String>,
}

impl From<&ValidParams> for Params {
    fn from(v: &ValidParams) -> Self {
        Self {
            name: Some(v.name.clone()),
            record_type: Some(v.record_type.to_string()),
            server: v.server.clone(),
        }
    }
}

pub(crate) fn parse_params(query: Option<&str>) -> Result<ValidParams, String> {
    let params: Params =
        serde_urlencoded::from_str(query.unwrap_or("")).map_err(|_| "invalid query parameters")?;
    let name = params.name.unwrap_or_default();
    if name.trim().is_empty() {
        return Err("missing 'name' parameter".into());
    }
    let rtype = params
        .record_type
        .as_deref()
        .and_then(|s| RecordType::from_str(s).ok())
        .unwrap_or(RecordType::A);
    let server = params.server.filter(|s| !s.is_empty());
    Ok(ValidParams {
        name,
        record_type: rtype,
        server,
    })
}

pub async fn handler(
    State(_state): State<AppState>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let raw_params: Params =
        serde_urlencoded::from_str(query.as_deref().unwrap_or("")).unwrap_or_default();

    if query.as_deref().unwrap_or("").is_empty() {
        return ui::dns::page(&Params::default(), None, None);
    }

    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return ui::dns::page(&raw_params, None, Some(&msg));
        }
    };

    let server_ref = params.server.as_deref();
    match ipgeom_query::dns::authoritative_query(&params.name, params.record_type, server_ref).await
    {
        Ok(res) => ui::dns::page(
            &Params::from(&params),
            Some((&res.authoritative_server, &res.records)),
            None,
        ),
        Err(e) => ui::dns::page(&Params::from(&params), None, Some(&e.to_string())),
    }
}
