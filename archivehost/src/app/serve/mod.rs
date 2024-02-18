use axum::{extract::Request, Router, ServiceExt};
use std::sync::Arc;
use tower::Layer as _;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};
use tracing::info;

use crate::{cli::ServeArgs, config::CONFIG};

mod download_queue;
#[cfg(not(debug_assertions))]
mod frontend;
mod graphql;
mod web;

struct StateInner {
    dl_q: download_queue::DownloadQueueController,
    conn: sea_orm::DatabaseConnection,
}
type AppState = Arc<StateInner>;

pub async fn serve(conn: sea_orm::DatabaseConnection, _args: ServeArgs) -> eyre::Result<()> {
    let state = StateInner {
        dl_q: download_queue::DownloadQueueController::start(
            conn.clone(),
            CONFIG.download.concurrency,
        ),
        conn,
    };
    let state = Arc::new(state);

    let router = Router::new()
        .nest("/web", web::router())
        .merge(graphql::router(state.clone()));

    #[cfg(not(debug_assertions))]
    let router = router.fallback(frontend::static_handler);

    let router = router.with_state(state);

    let app =
        NormalizePathLayer::trim_trailing_slash().layer(router.layer(TraceLayer::new_for_http()));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", CONFIG.serve.port)).await?;
    info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}
