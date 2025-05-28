mod routes;
mod ui;
mod util;

use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use axum::{routing::get, Router};
use ipgeom_rir::{Database, SqliteDb};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Clone)]
struct AppState {
    db: Arc<dyn Database>,
}

/// Run the HTTP server listening on `addr` using the SQLite database at `db_path`.
pub async fn run(addr: SocketAddr, db_path: &Path) -> Result<(), anyhow::Error> {
    let db = SqliteDb::open(db_path)?;
    let state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .route("/", get(routes::home::handler))
        .route("/myip", get(routes::myip::handler))
        .route("/lookup", get(routes::lookup::handler))
        .route("/dns", get(routes::dns::handler))
        .route("/whois", get(routes::whois::handler))
        .route("/rdap", get(routes::rdap::handler))
        .route("/ping", get(routes::ping::handler))
        .route("/traceroute", get(routes::traceroute::handler))
        .route("/cert", get(routes::cert::handler))
        .route("/api", get(routes::apidoc::handler))
        .route("/api/v1/query/dns", get(routes::api::dns::handler))
        .route("/api/v1/query/whois", get(routes::api::whois::handler))
        .route("/api/v1/query/rdap", get(routes::api::rdap::handler))
        .route(
            "/api/v1/query/domain-certificate",
            get(routes::api::domain_cert::handler),
        )
        .route("/api/v1/ping", get(routes::api::ping::handler))
        .route(
            "/api/v1/query/traceroute",
            get(routes::api::traceroute::handler),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("listening on {}", addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
