use std::sync::{Arc, Mutex};

use colored::Colorize as _;
use futures::StreamExt as _;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::signal;
use tracing::info;

use crate::{
    common::{
        download::{download_page, get_latest_pages_index, DownloadStatus},
        wayback_client::WaybackClient,
    },
    config::{cli::DownloadArgs, CONFIG},
};

pub async fn download(args: DownloadArgs) -> eyre::Result<()> {
    let client = WaybackClient::default();

    info!("Fetching index...");
    let index = get_latest_pages_index(&client, args.url, args.from, args.to).await?;
    info!("{} entries found.", index.len());

    let pb = ProgressBar::new(index.len() as u64);
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

    let client = Arc::new(client);

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

    let tasks = index
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

                let res = download_page(client.as_ref(), record).await;

                match res {
                    Ok(DownloadStatus::Done) => {
                        pb.println(format!(
                            "{}    {} [{}]",
                            " Done ".on_green(),
                            record.timestamp,
                            record.original,
                        ));
                        *count.done.lock().unwrap() += 1;
                    }
                    Ok(DownloadStatus::Skipped(reason)) => {
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
                            "{}   {} [{}]",
                            " Error ".on_red(),
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
        .buffer_unordered(CONFIG.download.concurrency)
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
