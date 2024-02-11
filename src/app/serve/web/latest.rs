use axum::{
    extract::{Path, Request},
    response::IntoResponse,
};
use http::StatusCode;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::app::serve::web::utils::find_latest_page;

#[tracing::instrument(skip(request), err(Debug, level = "warn"))]
pub async fn serve_site_latest(
    Path(url): Path<String>,
    request: Request,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let latest = find_latest_page(None, url)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Bad request: {}", e)))?;

    let Some((_latest_timestamp, latest_path)) = latest else {
        return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    };

    tracing::info!("Serving file: {:?}", latest_path);

    Ok(ServeFile::new(latest_path).oneshot(request).await)
}
