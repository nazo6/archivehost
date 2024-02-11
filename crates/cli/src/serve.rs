use axum::{body::to_bytes, http::StatusCode, middleware, response::Response, Router};
use bytes::BytesMut;
use std::path::Path;
use tower_http::services::ServeDir;

use crate::{ServeArgs, DEFAULT_SAVE_PATH};

async fn my_middleware(response: Response) -> Result<Response, (StatusCode, String)> {
    if response.headers().get("content-type") == Some(&"text/html".parse().unwrap()) {
        let (mut header, body) = response.into_parts();
        let bytes = to_bytes(body, usize::MAX).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to convert body to bytes".to_string(),
            )
        })?;
        let script = br#"
<script>
  if ('serviceWorker' in navigator) {
    console.log('service worker is active');
    navigator.serviceWorker.register('/sw.js').then(function (registration) {
      console.log('ServiceWorker registration successful with scope: ', registration.scope);
    }).catch(function (err) {
      console.log('ServiceWorker registration failed: ', err);
    });
  }
</script>
            "#;

        let mut new_bytes = BytesMut::from(&script[..]);
        new_bytes.extend(bytes);
        let new_bytes = new_bytes.freeze();
        if let Some(len) = header.headers.get_mut("content-length") {
            *len = new_bytes.len().into();
        }
        return Ok(Response::from_parts(
            header,
            axum::body::Body::from(new_bytes),
        ));
    }

    Ok(response)
}

pub async fn serve(args: ServeArgs) -> eyre::Result<()> {
    let serve_path = Path::new(DEFAULT_SAVE_PATH).join(args.host);

    let app = Router::new()
        .fallback_service(ServeDir::new(serve_path))
        .layer(middleware::map_response(my_middleware));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", args.port)).await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
