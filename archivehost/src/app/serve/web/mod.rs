use axum::{
    extract::Path,
    response::Redirect,
    routing::{get, Router},
};

use self::{latest::serve_site_latest, timestamp::serve_site_with_timestamp};

use super::AppState;

mod decode;
mod dummy_file;
mod latest;
mod serve_file;
mod timestamp;
mod utils;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/:site",
            get(|Path(site): Path<String>| async move {
                Redirect::to(&format!("/web/latest/{}", site))
            }),
        )
        .route("/latest/*url", get(serve_site_latest))
        .route("/:timestamp/*url", get(serve_site_with_timestamp))
}
