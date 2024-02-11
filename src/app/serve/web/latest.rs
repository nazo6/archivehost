use axum::{extract::Path, response::IntoResponse};
use http::StatusCode;

use crate::app::serve::web::{serve_file::serve_file, utils::find_latest_page};

#[tracing::instrument(err(Debug, level = "warn"))]
pub async fn serve_site_latest(Path(url): Path<String>) -> impl IntoResponse {
    let url = url
        .parse()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Bad request: {}", e)))?;
    let latest = find_latest_page(None, &url)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Bad request: {}", e)))?;

    let Some((_latest_timestamp, latest_path)) = latest else {
        return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    };

    tracing::debug!("Serving file: {:?}", latest_path);

    Ok(serve_file(&latest_path, &url, None).await)
}
