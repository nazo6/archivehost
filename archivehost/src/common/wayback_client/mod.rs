use std::time::Duration;

use http::StatusCode;
use reqwest::Response;
use serde::Serialize;

mod error;
use error::Result;

use super::timestamp::Timestamp;

pub struct WaybackClient {
    pub client: reqwest::Client,
    pub base_url: String,
}

impl Default for WaybackClient {
    fn default() -> Self {
        let client = reqwest::Client::builder()
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/115.0",
            )
            .build()
            .unwrap();
        let base_url = "https://web.archive.org".to_string();
        WaybackClient { client, base_url }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CdxRawOptions {
    pub url: String,
    pub limit: Option<u32>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub collapse: Option<String>,
    pub match_type: Option<CdxMatchType>,
    pub show_resume_key: Option<bool>,
    pub resume_key: Option<String>,
    pub filter: Option<String>,
    pub output: Option<String>,
}

#[derive(Debug)]
pub struct CdxOptions {
    pub url: String,
    pub limit: Option<u32>,
    pub from: Option<Timestamp>,
    pub to: Option<Timestamp>,
    pub collapse: Option<String>,
    pub match_type: Option<CdxMatchType>,
    pub filter: Option<String>,
    pub resume_key: Option<String>,
}

/// The default behavior is to return matches for an exact url. However, the cdx server can also return results matching a certain prefix,
/// a certain host or all subdomains by using the matchType= param.
///
/// The matchType may also be set implicitly by using wildcard '*' at end or beginning of the url:
/// - If url is ends in '/*', eg url=archive.org/* the query is equivalent to url=archive.org/&matchType=prefix
/// - if url starts with '*.', eg url=*.archive.org/ the query is equivalent to url=archive.org/&matchType=domain
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub enum CdxMatchType {
    /// matchType=exact (default if omitted) will return results matching exactly archive.org/about/
    Exact,
    /// matchType=prefix will return results for all results under the path archive.org/about/
    Prefix,
    /// matchType=host will return results from host archive.org
    Host,
    /// matchType=domain will return results from host archive.org and all subhosts *.archive.org
    Domain,
}

#[derive(Debug)]
pub struct CdxData {
    pub data: Vec<CdxLine>,
    pub resume_key: Option<String>,
}

#[derive(Debug)]
pub struct CdxLine {
    pub url_key: String,
    pub timestamp: Timestamp,
    pub original: String,
    pub mime: String,
    pub status_code: Option<StatusCode>,
    pub digest: String,
    pub length: String,
}

impl WaybackClient {
    pub async fn get_cdx_raw(&self, opts: CdxRawOptions) -> Result<Vec<Vec<String>>> {
        let params = serde_url_params::to_string(&opts)?;
        let url = format!("{}/cdx/search/cdx?{}", &self.base_url, &params);
        let json = self.client.get(url).send().await?.json().await?;

        Ok(json)
    }

    /// get cdx with resume key
    pub async fn get_cdx(&self, opts: CdxOptions) -> Result<Option<CdxData>> {
        let opts = CdxRawOptions {
            url: opts.url,
            limit: opts.limit,
            from: opts.from.map(|t| t.to_wb_ts()),
            to: opts.to.map(|t| t.to_wb_ts()),
            collapse: opts.collapse,
            match_type: opts.match_type,
            filter: opts.filter,
            show_resume_key: Some(true),
            resume_key: opts.resume_key,
            output: Some("json".to_string()),
        };
        let mut lines = self.get_cdx_raw(opts).await?;

        if lines.is_empty() {
            return Ok(None);
        }

        let resume_key = if lines.len() >= 3 {
            let has_resume_key =
                lines.last().unwrap().len() == 1 && lines[lines.len() - 2].is_empty();
            if has_resume_key {
                let resume_key = lines.pop().unwrap().pop().unwrap();
                lines.pop();
                Some(resume_key)
            } else {
                None
            }
        } else {
            None
        };

        let mut lines = lines.into_iter();

        let Some(first) = lines.next() else {
            return Ok(None);
        };
        if first
            != vec![
                "urlkey",
                "timestamp",
                "original",
                "mimetype",
                "statuscode",
                "digest",
                "length",
            ]
        {
            return Err(error::Error::Other(format!(
                "First line of CDX response is not as expected: {:?}",
                first
            )));
        }

        let mut data = vec![];
        for line in lines {
            if line.len() != 7 {
                return Err(error::Error::Other(
                    "CDX response line is not as expected".to_string(),
                ));
            }
            let mut line = line.into_iter();
            data.push(CdxLine {
                url_key: line.next().unwrap(),
                timestamp: Timestamp::from_wb_ts(&line.next().unwrap()).map_err(|e| {
                    error::Error::Other(format!("Failed to parse timestamp: {}", e))
                })?,
                original: line.next().unwrap(),
                mime: line.next().unwrap(),
                status_code: line.next().unwrap().parse().ok(),
                digest: line.next().unwrap(),
                length: line.next().unwrap(),
            });
        }

        Ok(Some(CdxData { resume_key, data }))
    }

    pub async fn get_page(&self, snapshot_date: &str, url: &str) -> Result<Response> {
        let url = format!("{}/web/{snapshot_date}id_/{url}", &self.base_url);
        let resp = self.client.get(url).send().await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_cdx() {
        let client = WaybackClient::default();
        let opts = CdxOptions {
            url: "example.com".to_string(),
            limit: Some(10),
            from: Some(Timestamp::from_wb_ts("20210101").unwrap()),
            to: Some(Timestamp::from_wb_ts("20210101").unwrap()),
            collapse: Some("urlkey".to_string()),
            match_type: Some(CdxMatchType::Exact),
            resume_key: None,
            filter: None,
        };
        let cdx_data = client.get_cdx(opts).await.unwrap();
        dbg!(&cdx_data);
        assert!(cdx_data.is_some());
    }

    #[tokio::test]
    async fn test_get_page() {
        let client = WaybackClient::default();
        let snapshot_date = "20210101000000";
        let url = "http://example.com";
        let resp = client.get_page(snapshot_date, url).await.unwrap();
        assert_eq!(resp.status(), 200);
    }
}
