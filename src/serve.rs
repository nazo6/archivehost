use axum::{
    extract::{Path, Request},
    response::Redirect,
    routing::get,
    Router,
};

use crate::ServeArgs;

mod asset;
mod web;

pub async fn serve(args: ServeArgs) -> eyre::Result<()> {
    let app = Router::new()
        .route("/sw.js", get(asset::sw_js))
        .route("/web", get(web::site_index))
        .route(
            "/web/:site",
            get(
                |Path(site): Path<String>| async move { Redirect::to(&format!("/web/{}/_", site)) },
            ),
        )
        .route(
            "/web/:site/_",
            get(|Path(site): Path<String>| async {
                web::redirect_to_latest(Path((site, "".to_string()))).await
            }),
        )
        .route("/web/:site/_/*path", get(web::redirect_to_latest))
        .route(
            "/web/:site/:timestamp",
            get(
                |Path((site, timestamp)): Path<(String, u64)>, request: Request| async move {
                    web::serve_site(Path((site, timestamp, "".to_string())), request).await
                },
            ),
        )
        .route("/web/:site/:timestamp/*path", get(web::serve_site));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", args.port)).await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
