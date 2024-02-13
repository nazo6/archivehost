use axum::{extract::Path, response::IntoResponse};
use http::StatusCode;

use crate::cli::serve::web::utils::parse_url;

use super::{serve_file::serve_file, utils::find_latest_page};

#[tracing::instrument(err(Debug, level = "warn"))]
pub async fn serve_site_latest(
    Path(url): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let url =
        parse_url(&url).map_err(|e| (StatusCode::BAD_REQUEST, format!("Bad request: {}", e)))?;
    let (_latest_ts, latest_path) = find_latest_page(None, &url)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Bad request: {}", e)))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Not found".to_string()))?;

    tracing::debug!("Serving file: {:?}", latest_path);

    Ok(serve_file(&latest_path, &url, "latest").await)
}
