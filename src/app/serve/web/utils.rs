use std::path::{Path, PathBuf};

use tokio::fs::metadata;
use tracing::debug;
use url::Url;

use crate::config::DATA_DIR;

/// !!! This must be called after host is checked to exist.
async fn find_path(timestamp: u64, url: &Url) -> Result<Option<PathBuf>, eyre::Report> {
    let timestamp = timestamp.to_string();

    let mut check_paths = vec![];
    let scheme = url.scheme();
    let scheme_alt = if scheme == "http" {
        "https"
    } else if scheme == "https" {
        "http"
    } else {
        return Err(eyre::eyre!("Unsupported scheme"));
    };

    let base = Path::new(DATA_DIR.get().unwrap());
    let path = Path::new(url.host_str().ok_or(eyre::eyre!("No host"))?)
        .join(Path::new(url.path()).strip_prefix("/")?);

    check_paths.push(base.join(&timestamp).join(scheme).join(&path));

    if !url.path().ends_with(".html") {
        check_paths.push(
            base.join(&timestamp)
                .join("http")
                .join(&path)
                .join("index.html"),
        );
    }

    check_paths.push(base.join(&timestamp).join(scheme_alt).join(&path));

    if !url.path().ends_with(".html") {
        check_paths.push(
            base.join(timestamp)
                .join("https")
                .join(&path)
                .join("index.html"),
        );
    }

    for path in check_paths {
        if let Ok(metadata) = metadata(&path).await {
            if metadata.is_file() {
                debug!("Found path: {:?}", path);
                return Ok(Some(path));
            }
        }
    }

    Ok(None)
}

/// Find the latest page for a site and path.
#[tracing::instrument(err)]
pub async fn find_latest_page(
    until: Option<u64>,
    url: String,
) -> eyre::Result<Option<(u64, PathBuf)>> {
    let url = url::Url::parse(&url)?;

    if url.host_str().is_none() {
        return Err(eyre::eyre!("No host in URL"));
    };

    if let Some(until) = until {
        if let Some(path) = find_path(until, &url).await? {
            return Ok(Some((until, path)));
        }
    }

    let mut latest: Option<(u64, PathBuf)> = None;

    let Ok(mut timestamp_folders) = tokio::fs::read_dir(&DATA_DIR.get().unwrap()).await else {
        return Ok(None);
    };
    while let Ok(Some(folder)) = timestamp_folders.next_entry().await {
        if let Some(timestamp) = folder.file_name().to_str() {
            if let Ok(timestamp) = timestamp.parse::<u64>() {
                if let Some(until) = until {
                    if timestamp > until {
                        continue;
                    }
                }
                if let Some(latest) = &latest {
                    if timestamp < latest.0 {
                        continue;
                    }
                }

                if let Some(path) = find_path(timestamp, &url).await? {
                    latest = Some((timestamp, path));
                }
            }
        }
    }

    Ok(latest)
}
