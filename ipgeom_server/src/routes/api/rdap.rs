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
    let params = match parse_params(query.as_deref()) {
        Ok(v) => v,
        Err(msg) => {
            return util::json_error(axum::http::StatusCode::BAD_REQUEST, &msg).into_response()
        }
    };

    match ipgeom_query::rdap(params.query_type).await {
        Ok(res) => Json(res).into_response(),
        Err(e) => {
            util::json_error(axum::http::StatusCode::BAD_REQUEST, &e.to_string()).into_response()
        }
    }
}
