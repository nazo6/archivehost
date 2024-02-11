use axum::response::IntoResponse;

pub async fn sw_js() -> impl IntoResponse {
    include_str!("../../../asset/sw.js")
}
