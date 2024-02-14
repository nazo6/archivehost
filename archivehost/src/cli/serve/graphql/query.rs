use async_graphql::types::connection::*;
use async_graphql::{Object, Result};

use crate::config::CONN;
use crate::db::archive;

pub struct QueryRoot;
#[Object]
impl QueryRoot {
    async fn site_list(&self, offset: Option<i32>, limit: Option<i32>) -> Result<Vec<String>> {
        let offset = offset.unwrap_or(0);
        let limit = if let Some(limit) = limit {
            limit.min(100).max(1)
        } else {
            100
        };

        todo!()
    }
}
