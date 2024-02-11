use axum::{
    extract::Path,
    response::{Html, IntoResponse, Redirect},
    routing::{get, Router},
};
use http::StatusCode;

use crate::config::DATA_DIR;

use self::{latest::serve_site_latest, timestamp::serve_site_with_timestamp};

mod latest;
mod timestamp;
mod utils;

pub fn route() -> Router {
    Router::new()
        .route("/", get(site_list))
        .route(
            "/:site",
            get(|Path(site): Path<String>| async move {
                Redirect::to(&format!("/web/{}/latest", site))
            }),
        )
        .route("/latest/*url", get(serve_site_latest))
        .route("/:timestamp/*url", get(serve_site_with_timestamp))
}

#[tracing::instrument(skip_all, err(Debug))]
pub async fn site_list() -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut folders = tokio::fs::read_dir(DATA_DIR.get().unwrap())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read data dir: {}", e),
            )
        })?;
    let mut lis = String::new();
    while let Ok(Some(folder)) = folders.next_entry().await {
        lis.push_str(&format!(
            "<li><a href=\"/web/{}\">{}</a></li>",
            folder.file_name().to_string_lossy(),
            folder.file_name().to_string_lossy()
        ));
    }

    let html = Html(
        include_str!("../../../../asset/siteIndexTemplate.html").replace("{{siteList}}", &lis),
    );

    Ok(html)
}
