use std::collections::HashMap;

use crate::common::{
    timestamp::Timestamp,
    wayback_client::{CdxLine, CdxMatchType, CdxOptions, WaybackClient},
};
use eyre::OptionExt;

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

pub async fn get_latest_page_cdx(
    client: &WaybackClient,
    url: String,
    from: Option<Timestamp>,
    to: Option<Timestamp>,
) -> eyre::Result<Option<CdxLine>> {
    let index = client
        .get_cdx(CdxOptions {
            url,
            limit: None,
            from,
            to,
            collapse: Some("digest".to_string()),
            match_type: Some(CdxMatchType::Exact),
            resume_key: None,
            filter: None,
        })
        .await?
        .ok_or_eyre("No index found")?;
    Ok(index
        .data
        .into_iter()
        .max_by(|v1, v2| v1.timestamp.cmp(&v2.timestamp)))
}
