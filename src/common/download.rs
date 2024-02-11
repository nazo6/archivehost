use std::{collections::HashMap, path::Path};

use super::wayback_client::{CdxLine, CdxMatchType, CdxOptions, WaybackClient};
use eyre::OptionExt;

use crate::config::{DATA_DIR, DEFAULT_IGNORE_MIME_TYPES};

/// Get the latest index of pages from the web archive
pub async fn get_latest_pages_index(
    client: &WaybackClient,
    url: String,
    from: Option<String>,
    to: Option<String>,
) -> eyre::Result<HashMap<String, CdxLine>> {
    let index = client
        .get_cdx(CdxOptions {
            url,
            limit: None,
            from,
            to,
            collapse: Some("digest".to_string()),
            match_type: Some(CdxMatchType::Prefix),
            resume_key: None,
            filter: None,
        })
        .await?
        .ok_or_eyre("No index found")?;

    let mut index_data_map: HashMap<String, CdxLine> = HashMap::new();
    for record in index.data {
        if let Some(status_code) = record.status_code {
            if !status_code.is_success() {
                continue;
            }
        }
        if record.status_code.is_none() {
            continue;
        }

        use std::collections::hash_map::Entry;
        match index_data_map.entry(record.original.clone()) {
            Entry::Occupied(v) => {
                if v.get().timestamp < record.timestamp {
                    *v.into_mut() = record;
                }
            }
            Entry::Vacant(v) => {
                v.insert(record);
            }
        }
    }

    Ok(index_data_map)
}

pub enum DownloadStatus {
    Done,
    Skipped(String),
}
/// Download and save page to DATA_DIR
pub async fn download_page(
    client: &WaybackClient,
    record: &CdxLine,
) -> Result<DownloadStatus, eyre::Report> {
    if DEFAULT_IGNORE_MIME_TYPES.contains(&record.mime.as_str()) {
        return Ok(DownloadStatus::Skipped(format!(
            "Ignored mime: {}",
            record.mime
        )));
    }

    let url = url::Url::parse(&record.original)?;
    let host = url.host_str().ok_or_else(|| eyre::eyre!("No host found"))?;
    let path = url.path().to_string();
    let path = urlencoding::decode(&path)?;
    let path = if path.ends_with('/') {
        path + "index.html"
    } else if record.mime.starts_with("text/html") && !path.ends_with(".html") {
        path + "/index.html"
    } else {
        path
    };

    let path = path.trim_start_matches('/');

    let save_path = Path::new(DATA_DIR.get().unwrap())
        .join(&record.timestamp)
        .join(url.scheme())
        .join(host)
        .join(path);

    if save_path.exists() {
        return Ok(DownloadStatus::Skipped("File already exists".to_string()));
    }

    let resp = client.get_page(&record.timestamp, &record.original).await?;
    let save_dir = save_path.parent().unwrap();
    tokio::fs::create_dir_all(&save_dir).await?;
    tokio::fs::write(&save_path, resp.bytes().await?).await?;

    Ok::<DownloadStatus, eyre::Report>(DownloadStatus::Done)
}
