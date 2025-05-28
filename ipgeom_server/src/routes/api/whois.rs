use axum::{
    extract::{RawQuery, State},
    response::IntoResponse,
    Json,
};

use crate::{routes::whois::parse_params, util, AppState};

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

    match ipgeom_query::domain_whois(&params.domain).await {
        Ok(res) => Json(res).into_response(),
        Err(e) => {
            util::json_error(axum::http::StatusCode::BAD_REQUEST, &e.to_string()).into_response()
        }
    }
}
