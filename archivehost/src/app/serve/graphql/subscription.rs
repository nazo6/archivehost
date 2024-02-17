use std::time::Duration;

use async_graphql::*;
use futures_util::Stream;

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn integers(&self) -> impl Stream<Item = i32> {
        let mut val = 0;
        async_stream::stream!({
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                val += 1;
                yield val;
            }
        })
    }
}
