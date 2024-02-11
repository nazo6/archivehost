use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use archivehost_core::{CdxLine, CdxMatchType, CdxOptions, WebArchiveClient};
use colored::Colorize;
use eyre::OptionExt;
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::signal;

use crate::{DownloadArgs, DEFAULT_SAVE_PATH};

static DEFAULT_IGNORE_MIME_TYPES: &[&str] = &["application/zip"];

pub async fn download(args: DownloadArgs) -> eyre::Result<()> {
    let client = WebArchiveClient::default();

    println!("Fetching index...");

    let index = client
        .get_cdx(CdxOptions {
            url: args.url.clone(),
            limit: None,
            from: args.from,
            to: args.to,
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

        let url_without_protocol = record
            .original
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .to_string();

        use std::collections::hash_map::Entry;
        match index_data_map.entry(url_without_protocol) {
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

    println!("{} entries found.", index_data_map.len());

    let client = Arc::new(client);

    let pb = ProgressBar::new(index_data_map.len() as u64);
    let pb_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) \n {msg}")?;
    pb.set_style(pb_style);

    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    {
        let pb = pb.clone();
        tokio::spawn(async move {
            match signal::ctrl_c().await {
                Ok(()) => {
                    pb.println("Shutting down...");
                }
                Err(err) => {
                    pb.println(format!("Shutting down... (error) : {}", err));
                }
            }
            shutdown_tx.send(true).unwrap();
        });
    }

    struct Count {
        done: Mutex<u64>,
        skipped: Mutex<u64>,
        error: Mutex<u64>,
    }
    let count = Arc::new(Count {
        done: Mutex::new(0),
        skipped: Mutex::new(0),
        error: Mutex::new(0),
    });

    let tasks = index_data_map
        .values()
        .map(|record| {
            let client = client.clone();
            let pb = pb.clone();
            let count = count.clone();
            let shutdown_rx = shutdown_rx.clone();
            async move {
                if *shutdown_rx.borrow() {
                    return;
                }

                enum Status {
                    Done,
                    Skipped(String),
                }
                let res = async move {
                    if DEFAULT_IGNORE_MIME_TYPES.contains(&record.mime.as_str()) {
                        return Ok(Status::Skipped(format!("Ignored mime: {}", record.mime)));
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

                    let save_path = Path::new(DEFAULT_SAVE_PATH).join(host).join(path);

                    if save_path.exists() {
                        return Ok(Status::Skipped("File already exists".to_string()));
                    }

                    let resp = client.get_page(&record.timestamp, &record.original).await?;
                    let save_dir = save_path.parent().unwrap();
                    tokio::fs::create_dir_all(&save_dir).await?;
                    // .wrap_err(format!("Failed to create dir: {:?}", save_dir))?;
                    tokio::fs::write(&save_path, resp.bytes().await?).await?;

                    Ok::<Status, eyre::Report>(Status::Done)
                }
                .await;

                match res {
                    Ok(Status::Done) => {
                        pb.println(format!(
                            "{} {} [{}]",
                            "  Done   ".on_green(),
                            record.timestamp,
                            record.original,
                        ));
                        *count.done.lock().unwrap() += 1;
                    }
                    Ok(Status::Skipped(reason)) => {
                        pb.println(format!(
                            "{} {} [{}]",
                            " Skipped ".on_blue(),
                            reason,
                            record.original
                        ));
                        *count.skipped.lock().unwrap() += 1;
                    }
                    Err(e) => {
                        pb.println(format!(
                            "{} {} [{}]",
                            "  Error  ".on_red(),
                            e,
                            record.original
                        ));
                        *count.error.lock().unwrap() += 1;
                    }
                }
                pb.set_message(format!(
                    "Done: {}, Skipped: {}, Error: {}",
                    *count.done.lock().unwrap(),
                    *count.skipped.lock().unwrap(),
                    *count.error.lock().unwrap()
                ));
                pb.inc(1);
            }
        })
        .collect::<Vec<_>>();

    futures::stream::iter(tasks)
        .buffer_unordered(args.concurrency as usize)
        .collect::<Vec<_>>()
        .await;

    if *shutdown_rx.borrow() {
        pb.println("Tasks stopped.");
        return Ok(());
    }

    pb.finish_with_message(format!(
        "Completed!: Done: {}, Skipped: {}, Error: {}",
        *count.done.lock().unwrap(),
        *count.skipped.lock().unwrap(),
        *count.error.lock().unwrap()
    ));

    Ok(())
}
