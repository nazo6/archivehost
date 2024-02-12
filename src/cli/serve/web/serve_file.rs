use std::path::Path;

use axum::{body::Body, response::IntoResponse};
use http::{header, StatusCode};
use tokio_util::io::ReaderStream;
use url::Url;

use crate::{common::timestamp::Timestamp, config::CONFIG};

/// Serve file but replace some URLs.
/// This is far from perfect but works well enough for now.
///
/// Imrovements ideas:
///    - Use service worker
///    - Use proxy
pub async fn serve_file(
    path: &Path,
    orig_url: &Url,
    timestamp: Option<Timestamp>,
) -> impl IntoResponse {
    let file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let Some(mime) = mime_guess::from_path(path).first_raw() else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to guess mime".to_string(),
        ));
    };

    let headers = [(header::CONTENT_TYPE, mime)];

    if mime == "text/html" {
        // let insert_code = include_str!("../../../../asset/loadSW.html");
        let text = tokio::fs::read_to_string(path).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read file: {}", e),
            )
        })?;
        // let new_text = format!("{}{}", insert_code, text);
        let Some(host) = orig_url.host_str() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "No host in request".to_string(),
            ));
        };

        let new_text = text.replace(&format!("https://{host}"), &format!("http://{host}"));

        let new_text = new_text.replace(
            &format!("http://{host}"),
            &format!(
                "http://{}/web/{}/{}://{}",
                CONFIG.serve.get_host(),
                timestamp
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "latest".to_string()),
                orig_url.scheme(),
                host
            ),
        );
        Ok((headers, new_text.into()))
    } else {
        let stream = ReaderStream::new(file);
        let body = Body::from_stream(stream);

        Ok((headers, body))
    }
}
