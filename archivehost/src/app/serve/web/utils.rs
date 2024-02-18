use std::path::PathBuf;

use db::entity::archive;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait as _, QueryFilter as _, QueryOrder as _,
    QueryTrait as _,
};
use url::Url;

use crate::{common::timestamp::Timestamp, config::CONFIG};

async fn find(
    conn: &DatabaseConnection,
    until: Option<&Timestamp>,
    host: &str,
    path: &str,
) -> Result<Option<archive::Model>, eyre::Error> {
    let find = archive::Entity::find()
        .filter(archive::Column::UrlScheme.is_in(vec!["http", "https"]))
        .filter(archive::Column::UrlHost.eq(host))
        .filter(archive::Column::UrlPath.eq(path));

    let find = if let Some(until) = until {
        find.filter(archive::Column::Timestamp.lte(until.unix_time()))
    } else {
        find
    };

    let find = find.order_by_desc(archive::Column::Timestamp);

    let data = find.one(conn).await?;

    let Some(data) = data else {
        return Ok(None);
    };

    Ok(Some(data))
}

/// Find the latest page for a site and path.
#[tracing::instrument(err)]
pub async fn find_latest_page(
    conn: &DatabaseConnection,
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

    let data = if let Some(data) = find(conn, until, url_host, url.path()).await? {
        data
    } else if let Some(data) = find(conn, until, url_host, url_with_index_html.path()).await? {
        data
    } else if let Some(data) = find(conn, until, url_host, url_alt.path()).await? {
        data
    } else {
        return Ok(None);
    };

    Ok(Some((
        Timestamp::from_unix_time(data.timestamp)?,
        download_dir.join(data.save_path),
    )))
}

pub fn parse_url(url: &str) -> Result<Url, eyre::Error> {
    let url = if !url.starts_with("http") {
        format!("http://{}", url)
    } else {
        url.to_string()
    };
    Url::parse(&url).map_err(|e| eyre::eyre!("Failed to parse url: {}", e))
}
