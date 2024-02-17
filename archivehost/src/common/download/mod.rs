use std::path::Path;

use db::entity::archive::{ActiveModel as DbArchiveActiveModel, Entity as DbArchive};
use sea_orm::{EntityTrait as _, Set};

use crate::{config::CONFIG, constant::CONN};

use super::{timestamp::Timestamp, wayback_client::WaybackClient};

pub mod cdx;

pub enum DownloadResult {
    Done,
    Skipped(String),
    FixDb(String),
}

/// Download page and insert info to db.
pub async fn download_and_save_page(
    client: &WaybackClient,
    url_str: &str,
    mime: &str,
    timestamp: &Timestamp,
    status_code: Option<http::StatusCode>,
) -> Result<DownloadResult, eyre::Report> {
    if CONFIG
        .download
        .ignored_mime_types
        .iter()
        .any(|v| mime.starts_with(v))
    {
        return Ok(DownloadResult::Skipped(format!("Ignored mime: {}", mime)));
    }

    let url = url::Url::parse(url_str)?;
    let url_host = url.host_str().ok_or_else(|| eyre::eyre!("No host found"))?;
    let url_path = url.path();
    let url_scheme = url.scheme();

    let file_path = {
        let path = urlencoding::decode(url_path)?;
        let path = if path.ends_with('/') {
            path + "index.html"
        } else if mime.starts_with("text/html") && !path.ends_with(".html") {
            path + "/index.html"
        } else {
            path
        };

        let path = path.trim_start_matches('/').to_string();
        path
    };

    let timestamp_str = timestamp.to_string();
    let save_path_rel = Path::new(&timestamp_str)
        .join(url.scheme())
        .join(url_host)
        .join(file_path);
    let save_path = Path::new(&CONFIG.download_dir()).join(&save_path_rel);

    if DbArchive::find_by_id((
        url_scheme.to_string(),
        url_host.to_string(),
        url_path.to_string(),
        timestamp.unix_time(),
    ))
    .one(&*CONN)
    .await?
    .is_some()
    {
        return Ok(DownloadResult::Skipped("Already exists in db".to_string()));
    }

    if tokio::fs::try_exists(&save_path).await? {
        DbArchive::insert(DbArchiveActiveModel {
            url_scheme: Set(url_scheme.to_string()),
            url_host: Set(url_host.to_string()),
            url_path: Set(url_path.to_string()),
            timestamp: Set(timestamp.unix_time()),
            mime: Set(mime.to_string()),
            status: Set(status_code.map(|v| v.as_u16() as i32)),
            save_path: Set(save_path_rel.to_string_lossy().to_string()),
        })
        .exec(&*CONN)
        .await?;
        return Ok(DownloadResult::FixDb(
            "File already exists. Data is inserted to db.".to_string(),
        ));
    }

    let resp = client.get_page(&timestamp_str, url_str).await?;
    let save_dir = save_path.parent().unwrap();
    tokio::fs::create_dir_all(&save_dir).await?;
    tokio::fs::write(&save_path, resp.bytes().await?).await?;

    DbArchive::insert(DbArchiveActiveModel {
        url_scheme: Set(url_scheme.to_string()),
        url_host: Set(url_host.to_string()),
        url_path: Set(url_path.to_string()),
        timestamp: Set(timestamp.unix_time()),
        mime: Set(mime.to_string()),
        status: Set(status_code.map(|v| v.as_u16() as i32)),
        save_path: Set(save_path_rel.to_string_lossy().to_string()),
    })
    .exec(&*CONN)
    .await?;

    Ok::<DownloadResult, eyre::Report>(DownloadResult::Done)
}
