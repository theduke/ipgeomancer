use crate::{routes::rdap::parse_params, util, AppState};
use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
    Json,
};

pub async fn handler(
    State(_state): State<AppState>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    // parse parameters for error reporting only
    match parse_params(query.as_deref()) {
        Ok(valid) => match ipgeom_query::rdap(valid.query_type).await {
            Ok(res) => Json(res).into_response(),
            Err(e) => util::json_error(axum::http::StatusCode::BAD_REQUEST, &e.to_string())
                .into_response(),
        },
        Err(msg) => util::json_error(axum::http::StatusCode::BAD_REQUEST, &msg).into_response(),
    }
}
