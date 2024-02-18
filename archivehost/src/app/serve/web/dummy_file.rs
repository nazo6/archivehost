use axum::response::IntoResponse;
use http::{header, StatusCode};
use mime_guess::mime;
use tracing::debug;
use url::Url;

pub async fn serve_dummy_file(web_url: &Url) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(mime) = mime_guess::from_path(web_url.path()).first() else {
        return Err((
            StatusCode::NOT_FOUND,
            "Not found (dummy: couldn't detect mime)".to_string(),
        ));
    };

    let res = if mime == mime::IMAGE_PNG || mime == mime::IMAGE_JPEG || mime == mime::IMAGE_GIF {
        let headers = [(header::CONTENT_TYPE, mime::IMAGE_PNG.to_string())];
        (
            headers,
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/asset/dummy/dummy.png"
            ))
            .into_response(),
        )
    } else {
        return Err((
            StatusCode::NOT_FOUND,
            "Not found (dummy: dummy file for this mime is not found)".to_string(),
        ));
    };

    debug!(
        "Serving dummy file for {:?} with mime {:?}",
        web_url.to_string(),
        mime
    );

    Ok(res)
}
