use crate::config::CONFIG;

use db::entity::archive::Entity as DbArchive;
use sea_orm::EntityTrait;

pub async fn fixdb(conn: &sea_orm::DatabaseConnection) -> eyre::Result<()> {
    let entries = DbArchive::find().all(conn).await?;
    let download_dir = CONFIG.download_dir();
    for entry in entries {
        let path = entry.save_path;
        if tokio::fs::metadata(download_dir.join(&path)).await.is_err() {
            match DbArchive::delete_by_id((
                entry.url_scheme,
                entry.url_host,
                entry.url_path,
                entry.timestamp,
            ))
            .exec(conn)
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
