use axum::response::IntoResponse;
use http::{header, StatusCode};
use mime_guess::mime;
use url::Url;

pub async fn serve_dummy_file(web_url: &Url) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(mime) = mime_guess::from_path(web_url.path()).first() else {
        return Err((
            StatusCode::NOT_FOUND,
            "Not found (dummy: couldn't detect mime)".to_string(),
        ));
    };

    if mime == mime::IMAGE_PNG || mime == mime::IMAGE_JPEG || mime == mime::IMAGE_GIF {
        let headers = [(header::CONTENT_TYPE, mime::IMAGE_PNG.to_string())];
        return Ok((
            headers,
            include_bytes!("../../../../asset/dummy/dummy.png").into_response(),
        ));
    }

    Err((
        StatusCode::NOT_FOUND,
        "Not found (dummy: dummy file for this meme is not found)".to_string(),
    ))
}
