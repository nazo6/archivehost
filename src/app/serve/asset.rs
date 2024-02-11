use axum::response::IntoResponse;
use http::header;

pub async fn sw_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript")],
        include_str!("../../../asset/sw.js"),
    )
}
