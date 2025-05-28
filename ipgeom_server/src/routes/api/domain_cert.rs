use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::{routes::cert::parse_params, util, AppState};

#[derive(Serialize)]
pub struct CertResponse {
    subject: String,
    issuer: String,
    not_before: String,
    not_after: String,
    valid: bool,
}

pub async fn handler(
    State(_state): State<AppState>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return util::json_error(axum::http::StatusCode::BAD_REQUEST, &msg).into_response()
        }
    };

    match ipgeom_query::fetch_certificate(&params.domain).await {
        Ok(info) => Json(CertResponse {
            subject: info.subject,
            issuer: info.issuer,
            not_before: info.not_before,
            not_after: info.not_after,
            valid: info.valid,
        })
        .into_response(),
        Err(e) => {
            util::json_error(axum::http::StatusCode::BAD_REQUEST, &e.to_string()).into_response()
        }
    }
}
