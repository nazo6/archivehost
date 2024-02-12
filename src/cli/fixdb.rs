use crate::{
    config::{CONFIG, CONN},
    db::archive,
};

pub async fn fixdb() -> eyre::Result<()> {
    let all_entries = CONN.archive().find_many(vec![]).exec().await?;
    let download_dir = CONFIG.download_dir();
    for entry in all_entries {
        let path = entry.save_path;
        if tokio::fs::metadata(download_dir.join(&path)).await.is_err() {
            match CONN
                .archive()
                .delete(archive::url_scheme_url_host_url_path_timestamp(
                    entry.url_scheme,
                    entry.url_host,
                    entry.url_path,
                    entry.timestamp,
                ))
                .exec()
                .await
            {
                Ok(_) => {
                    println!("Deleted: {:?}", path);
                }
                Err(e) => {
                    println!("Failed to delete: {:?}", path);
                    println!("{:?}", e);
                }
            }
        }
    }

    Ok(())
}
