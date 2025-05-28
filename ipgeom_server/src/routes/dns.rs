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
        return ui::dns::page(None, None, None, None, None);
    }

    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return ui::dns::page(
                raw_params.name.as_deref(),
                raw_params.record_type.as_deref(),
                raw_params.server.as_deref(),
                None,
                Some(&msg),
            )
        }
    };

    let server_ref = params.server.as_deref();
    match ipgeom_query::dns::authoritative_query(&params.name, params.record_type, server_ref).await
    {
        Ok(res) => {
            let rtype = params.record_type.to_string();
            ui::dns::page(
                Some(&params.name),
                Some(&rtype),
                server_ref,
                Some((&res.authoritative_server, &res.records)),
                None,
            )
        }
        Err(e) => {
            let rtype = params.record_type.to_string();
            ui::dns::page(
                Some(&params.name),
                Some(&rtype),
                server_ref,
                None,
                Some(&e.to_string()),
            )
        }
    }
}
