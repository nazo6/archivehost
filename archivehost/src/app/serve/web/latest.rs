use axum::extract::State;
use axum::{extract::Path, response::IntoResponse};
use http::StatusCode;

use crate::app::serve::AppState;

use super::{dummy_file::serve_dummy_file, utils::parse_url};

use super::{serve_file::serve_file, utils::find_latest_page};

#[tracing::instrument(skip(state), err(Debug, level = "warn"))]
pub async fn serve_site_latest(
    Path(url): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let url =
        parse_url(&url).map_err(|e| (StatusCode::BAD_REQUEST, format!("Bad request: {}", e)))?;
    let latest = find_latest_page(&state.conn, None, &url)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Bad request: {}", e)))?;

    let Some((latest_timestamp, latest_path)) = latest else {
        return Ok(serve_dummy_file(&url).await.into_response());
        // return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    };

    tracing::debug!(
        "Serving file at {:?} (request: latest):\n\t{:?} ",
        latest_timestamp,
        latest_path
    );

    Ok(serve_file(&latest_path, &url, "latest")
        .await
        .into_response())
}
