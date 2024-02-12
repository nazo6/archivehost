use axum::{
    extract::Path,
    response::{IntoResponse, Redirect, Response},
};
use http::StatusCode;

use super::{serve_file::serve_file, utils::find_latest_page};

#[tracing::instrument(err(Debug, level = "warn"))]
pub async fn serve_site_with_timestamp(
    Path((timestamp, url)): Path<(u64, String)>,
) -> Result<Response, (StatusCode, String)> {
    let url = url
        .parse()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Bad request: {}", e)))?;
    let latest = find_latest_page(Some(timestamp), &url)
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

    if latest_timestamp != timestamp {
        return Ok(Redirect::to(&format!("/web/{}/{}", latest_timestamp, url)).into_response());
    }

    tracing::debug!("Serving file: {:?}", latest_path);

    Ok(serve_file(&latest_path, &url, Some(timestamp))
        .await
        .into_response())
}
