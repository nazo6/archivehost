use tokio::sync::mpsc;

use super::DownloadQueueController;

impl DownloadQueueController {
    pub fn subscribe_changes(&self) -> mpsc::Receiver<()> {
        let (tx, rx) = mpsc::channel::<()>(1);
        {
            let task_completed_notifier = self.state_changed_notifier.clone();
            let new_task_notifier = self.new_task_notifier.clone();
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        _ = task_completed_notifier.notified() => {}
                        _ = new_task_notifier.notified() => {}
                    }
                    if tx.send(()).await.is_err() {
                        break;
                    }
                }
            });
        }
        rx
    }
}
