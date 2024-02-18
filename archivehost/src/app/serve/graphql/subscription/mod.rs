use async_graphql::MergedSubscription;

mod download_queue;

#[derive(MergedSubscription, Default)]
pub struct Subscription(download_queue::DownloadQueueSubscription);
