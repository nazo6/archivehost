use axum::{
    extract::Path,
    response::{IntoResponse, Response},
};
use http::StatusCode;

use crate::common::timestamp::Timestamp;

use super::{serve_file::serve_file, utils::find_latest_page};

#[tracing::instrument(err(Debug, level = "warn"))]
pub async fn serve_site_with_timestamp(
    Path((timestamp, url)): Path<(u64, String)>,
) -> Result<Response, (StatusCode, String)> {
    let timestamp = Timestamp::from_wb_ts(&timestamp.to_string())
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid timestamp: {}", e)))?;
    let url = url
        .parse()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Bad request: {}", e)))?;
    let latest = find_latest_page(Some(&timestamp), &url)
        .await
        .map_err(|_e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No page found".to_string(),
            )
        })?;

    let Some((latest_timestamp, latest_path)) = latest else {
        return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    };

    tracing::debug!("Serving file: {:?} ({:?})", latest_path, latest_timestamp);

    Ok(serve_file(&latest_path, &url, Some(timestamp))
        .await
        .into_response())
}
