use axum::{
    extract::{Path, Request},
    response::{Html, IntoResponse, Redirect, Response},
};
use http::StatusCode;
use tower::util::ServiceExt;
use tower_http::services::ServeFile;

use crate::config::DATA_DIR;

pub async fn site_index() -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut folders = tokio::fs::read_dir(DATA_DIR.get().unwrap())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read data dir: {}", e),
            )
        })?;
    let mut lis = String::new();
    while let Ok(Some(folder)) = folders.next_entry().await {
        lis.push_str(&format!(
            "<li><a href=\"/web/{}\">{}</a></li>",
            folder.file_name().to_string_lossy(),
            folder.file_name().to_string_lossy()
        ));
    }

    let html =
        Html(include_str!("../../asset/siteIndexTemplate.html").replace("{{siteList}}", &lis));

    Ok(html)
}

async fn find_latest_page(site: &str, until: Option<u64>, path: &str) -> eyre::Result<Option<u64>> {
    let site_dir = DATA_DIR.get().unwrap().join(site);

    if let Some(until) = until {
        if site_dir.join(until.to_string()).join(path).exists() {
            return Ok(Some(until));
        }
    }

    let mut latest: Option<u64> = None;

    let mut time_folders = tokio::fs::read_dir(&site_dir).await?;
    while let Ok(Some(folder)) = time_folders.next_entry().await {
        if let Some(folder_name) = folder.file_name().to_str() {
            if let Ok(timestamp) = folder_name.parse::<u64>() {
                if let Some(until) = until {
                    if timestamp > until {
                        continue;
                    }
                }
                if let Some(latest) = latest {
                    if timestamp < latest {
                        continue;
                    }
                }
                let path = folder.path().join(path);
                if path.exists() {
                    latest = Some(timestamp);
                }
            }
        }
    }

    Ok(latest)
}

#[tracing::instrument(skip(request), err(Debug))]
pub async fn serve_site(
    Path((site, timestamp, path)): Path<(String, u64, String)>,
    request: Request,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let latest = find_latest_page(&site, Some(timestamp), &path)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to find latest page: {}", e),
            )
        })?;

    let Some(latest) = latest else {
        return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    };

    let file_path = format!(
        "{}/{}/{}/{}",
        DATA_DIR.get().unwrap().to_string_lossy(),
        site,
        latest,
        path
    );

    tracing::info!("Serving file: {}", file_path);

    Ok(ServeFile::new(file_path).oneshot(request).await)
}

pub async fn redirect_to_latest(
    Path((site, path)): Path<(String, String)>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let latest = find_latest_page(&site, None, &path).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to find latest page: {}", e),
        )
    })?;
    let Some(latest) = latest else {
        return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    };

    Ok(Redirect::to(&format!("/web/{}/{}/{}", site, latest, path)))
}
