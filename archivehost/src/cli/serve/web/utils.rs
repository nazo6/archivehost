use std::path::PathBuf;

use url::Url;

use crate::{
    common::timestamp::Timestamp,
    config::{CONFIG, CONN},
    db::archive,
};

async fn find(
    until: Option<&Timestamp>,
    host: &str,
    path: &str,
) -> Result<Option<archive::Data>, eyre::Error> {
    let mut find = vec![
        archive::url_scheme::in_vec(vec!["http".to_string(), "https".to_string()]),
        archive::url_host::equals(host.to_string()),
        archive::url_path::equals(path.to_string()),
    ];
    if let Some(until) = until {
        find.push(archive::timestamp::lte(until.0.into()))
    };

    let mut data = CONN
        .archive()
        .find_many(find)
        .order_by(archive::timestamp::order(crate::db::SortOrder::Desc))
        .take(1)
        .exec()
        .await?;

    let Some(data) = data.pop() else {
        return Ok(None);
    };

    Ok(Some(data))
}

/// Find the latest page for a site and path.
#[tracing::instrument(err)]
pub async fn find_latest_page(
    until: Option<&Timestamp>,
    url: &Url,
) -> eyre::Result<Option<(Timestamp, PathBuf)>> {
    let url_scheme = url.scheme();
    if url_scheme != "http" && url_scheme != "https" {
        return Err(eyre::eyre!("Unsupported scheme"));
    };
    let url_host = url.host_str().ok_or_else(|| eyre::eyre!("No host found"))?;

    let url_with_index_html = {
        let mut url = url.clone();
        let path = url.path();
        let path = if path.ends_with('/') {
            format!("{}index.html", path)
        } else {
            format!("{}/index.html", path)
        };
        url.set_path(&path);
        url
    };
    let url_alt = {
        let mut url = url.clone();
        let path = url.path();
        let path = if path.ends_with('/') {
            path.strip_suffix('/').unwrap().to_string()
        } else {
            format!("{}/", path)
        };
        url.set_path(&path);
        url
    };

    let download_dir = CONFIG.download_dir();

    if let Some(data) = find(until, url_host, url.path()).await? {
        return Ok(Some((
            Timestamp(data.timestamp.to_utc()),
            download_dir.join(data.save_path),
        )));
    }

    if let Some(data) = find(until, url_host, url_with_index_html.path()).await? {
        return Ok(Some((
            Timestamp(data.timestamp.to_utc()),
            download_dir.join(data.save_path),
        )));
    }

    if let Some(data) = find(until, url_host, url_alt.path()).await? {
        return Ok(Some((
            Timestamp(data.timestamp.to_utc()),
            download_dir.join(data.save_path),
        )));
    }

    Ok(None)
}

pub fn parse_url(url: &str) -> Result<Url, eyre::Error> {
    let url = if !url.starts_with("http") {
        format!("http://{}", url)
    } else {
        url.to_string()
    };
    Url::parse(&url).map_err(|e| eyre::eyre!("Failed to parse url: {}", e))
}

#[tracing::instrument(err)]
pub fn parse_timestamp(timestamp_str: &str) -> Result<Timestamp, eyre::Error> {
    if let Ok(ts) = Timestamp::from_wb_ts(timestamp_str) {
        Ok(ts)
    } else if let Ok(ts) = Timestamp::from_date(timestamp_str) {
        Ok(ts)
    } else if let Ok(ts) = Timestamp::from_year(timestamp_str) {
        Ok(ts)
    } else {
        Err(eyre::eyre!("Invalid timestamp"))
    }
}
