use axum::{extract::Request, routing::get, Router, ServiceExt};
use once_cell::sync::OnceCell;
use tower::Layer as _;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};
use tracing::info;

use super::ServeArgs;

mod asset;
mod web;

static HOST: OnceCell<String> = OnceCell::new();

pub async fn serve(args: ServeArgs) -> eyre::Result<()> {
    let app = NormalizePathLayer::trim_trailing_slash().layer(
        Router::new()
            .route("/sw.js", get(asset::sw_js))
            .nest("/web", web::route())
            .layer(TraceLayer::new_for_http()),
    );

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", args.port)).await?;
    info!("Listening on {}", listener.local_addr()?);
    HOST.set(format!("localhost:{}", args.port).to_string())
        .unwrap();
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}
