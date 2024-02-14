use axum::{extract::Request, Router, ServiceExt};
use tower::Layer as _;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};
use tracing::info;

use crate::config::{cli::ServeArgs, CONFIG};

#[cfg(not(debug_assertions))]
mod frontend;
mod graphql;
mod web;

pub async fn serve(_args: ServeArgs) -> eyre::Result<()> {
    let router = Router::new()
        .nest("/web", web::route())
        .merge(graphql::router());

    #[cfg(not(debug_assertions))]
    let router = router.fallback(frontend::static_handler);

    let app =
        NormalizePathLayer::trim_trailing_slash().layer(router.layer(TraceLayer::new_for_http()));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", CONFIG.serve.port)).await?;
    info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}
