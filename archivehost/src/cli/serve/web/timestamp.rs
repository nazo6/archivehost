use axum::{
    extract::Path,
    response::{IntoResponse, Response},
};
use http::StatusCode;

use crate::{
    cli::serve::web::{dummy_file::serve_dummy_file, utils::parse_url},
    common::timestamp::Timestamp,
};

use super::{serve_file::serve_file, utils::find_latest_page};

#[tracing::instrument(err(Debug, level = "warn"))]
pub async fn serve_site_with_timestamp(
    Path((timestamp_str, url)): Path<(String, String)>,
) -> Result<Response, (StatusCode, String)> {
    let timestamp = Timestamp::from_str(&timestamp_str)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid timestamp: {}", e)))?;
    let url =
        parse_url(&url).map_err(|e| (StatusCode::BAD_REQUEST, format!("Bad request: {}", e)))?;
    let latest = find_latest_page(Some(&timestamp), &url)
        .await
        .map_err(|_e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No page found".to_string(),
            )
        })?;

    let Some((latest_timestamp, latest_path)) = latest else {
        return Ok(serve_dummy_file(&url).await.into_response());
        // return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    };

    tracing::debug!(
        "Serving file at {:?} (request: {:?}):\n\t{:?} ",
        latest_timestamp,
        timestamp_str,
        latest_path
    );

    Ok(serve_file(&latest_path, &url, &timestamp_str)
        .await
        .into_response())
}
