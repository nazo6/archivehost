use std::sync::Arc;

use tokio::sync::Notify;

use crate::common::wayback_client::WaybackClient;

mod mutate;
mod subscribe;
mod task_executor;

pub use mutate::*;

/// NOTE:
/// ダウンロードキューを管理する構造体。ダウンロードキューは当初メモリに保存しようとしていたが結構つらかったためデータベースに保存することにした。
/// なのでこの構造体を使わなくてもデータベースに直接クエリすることでダウンロードキューを管理できる。
/// しかし、データ変更時の通知機能を正常に機能させるため、キューをmutateする時は直接行うのではなく
/// まずこの構造体に変更後通知を行うメソッドを実装してからそのメソッドを呼ぶべきである。
pub struct DownloadQueueController {
    // Notify downloader when new tasks are available
    new_task_notifier: Arc<Notify>,
    client: Arc<WaybackClient>,
    state_changed_notifier: Arc<Notify>,
    conn: sea_orm::DatabaseConnection,
}

impl DownloadQueueController {
    pub fn start(conn: sea_orm::DatabaseConnection, concurrency: usize) -> Self {
        let new_task_notifier = Arc::new(Notify::new());
        let task_completed_notifier = Arc::new(Notify::new());
        let client = Arc::new(WaybackClient::default());
        {
            let new_task_notifier = new_task_notifier.clone();
            let task_completed_notifier = task_completed_notifier.clone();
            let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
            let client = client.clone();
            let conn = conn.clone();
            tokio::spawn(async move {
                task_executor::start_task_executor(
                    conn,
                    new_task_notifier,
                    task_completed_notifier,
                    semaphore,
                    client,
                )
                .await;
            });
        }
        Self {
            new_task_notifier,
            conn,
            state_changed_notifier: task_completed_notifier,
            client,
        }
    }
}
