use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::{routes::password_hash::parse_params, util, AppState};

#[derive(Serialize)]
struct PasswordHashResponse {
    method: String,
    hash: String,
}

pub async fn handler(
    State(_state): State<AppState>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return util::json_error(axum::http::StatusCode::BAD_REQUEST, &msg).into_response();
        }
    };

    match ipgeom_query::generate_bcrypt_hash(&params.password) {
        Ok(hash) => Json(PasswordHashResponse {
            method: params.method,
            hash,
        })
        .into_response(),
        Err(e) => {
            util::json_error(axum::http::StatusCode::BAD_REQUEST, &e.to_string()).into_response()
        }
    }
}
