use std::{collections::HashMap, path::Path};

use crate::config::CONFIG;
use crate::constant::CONN;

use super::{
    timestamp::Timestamp,
    wayback_client::{CdxLine, CdxMatchType, CdxOptions, WaybackClient},
};
use db::entity::archive::{ActiveModel as DbArchiveActiveModel, Entity as DbArchive};
use eyre::OptionExt;
use sea_orm::{EntityTrait, Set};

/// Get the latest index of pages from the web archive
pub async fn get_latest_pages_index(
    client: &WaybackClient,
    url: String,
    from: Option<Timestamp>,
    to: Option<Timestamp>,
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
    /// Download skipped but added item to db
    FixDb(String),
}
/// Download and save page to DOWNLOAD_DIR
pub async fn download_page(
    client: &WaybackClient,
    record: &CdxLine,
) -> Result<DownloadStatus, eyre::Report> {
    if CONFIG
        .download
        .ignored_mime_types
        .iter()
        .any(|v| record.mime.starts_with(v))
    {
        return Ok(DownloadStatus::Skipped(format!(
            "Ignored mime: {}",
            record.mime
        )));
    }

    let url = url::Url::parse(&record.original)?;
    let url_host = url.host_str().ok_or_else(|| eyre::eyre!("No host found"))?;
    let url_path = url.path();
    let url_scheme = url.scheme();

    let file_path = {
        let path = urlencoding::decode(url_path)?;
        let path = if path.ends_with('/') {
            path + "index.html"
        } else if record.mime.starts_with("text/html") && !path.ends_with(".html") {
            path + "/index.html"
        } else {
            path
        };

        let path = path.trim_start_matches('/').to_string();
        path
    };

    let timestamp_str = record.timestamp.to_string();
    let save_path_rel = Path::new(&timestamp_str)
        .join(url.scheme())
        .join(url_host)
        .join(file_path);
    let save_path = Path::new(&CONFIG.download_dir()).join(&save_path_rel);

    if DbArchive::find_by_id((
        url_scheme.to_string(),
        url_host.to_string(),
        url_path.to_string(),
        record.timestamp.unix_time(),
    ))
    .one(&*CONN)
    .await?
    .is_some()
    {
        return Ok(DownloadStatus::Skipped("Already exists in db".to_string()));
    }

    if tokio::fs::try_exists(&save_path).await? {
        DbArchive::insert(DbArchiveActiveModel {
            url_scheme: Set(url_scheme.to_string()),
            url_host: Set(url_host.to_string()),
            url_path: Set(url_path.to_string()),
            timestamp: Set(record.timestamp.unix_time()),
            mime: Set(record.mime.clone()),
            status: Set(record.status_code.map(|v| v.as_u16() as i32)),
            save_path: Set(save_path_rel.to_string_lossy().to_string()),
        })
        .exec(&*CONN)
        .await?;
        return Ok(DownloadStatus::FixDb(
            "File already exists. Data is inserted to db.".to_string(),
        ));
    }

    let resp = client.get_page(&timestamp_str, &record.original).await?;
    let save_dir = save_path.parent().unwrap();
    tokio::fs::create_dir_all(&save_dir).await?;
    tokio::fs::write(&save_path, resp.bytes().await?).await?;

    DbArchive::insert(DbArchiveActiveModel {
        url_scheme: Set(url_scheme.to_string()),
        url_host: Set(url_host.to_string()),
        url_path: Set(url_path.to_string()),
        timestamp: Set(record.timestamp.unix_time()),
        mime: Set(record.mime.clone()),
        status: Set(record.status_code.map(|v| v.as_u16() as i32)),
        save_path: Set(save_path_rel.to_string_lossy().to_string()),
    })
    .exec(&*CONN)
    .await?;

    Ok::<DownloadStatus, eyre::Report>(DownloadStatus::Done)
}
