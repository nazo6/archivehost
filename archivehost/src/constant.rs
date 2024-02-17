use once_cell::sync::Lazy;
use sea_orm::{Database, DatabaseConnection};

use crate::config::CONFIG;

pub static CONN: Lazy<DatabaseConnection> = Lazy::new(|| {
    futures::executor::block_on(async {
        let url = format!(
            "sqlite://{}/db.sqlite?mode=rwc",
            CONFIG.root.to_string_lossy()
        );
        Database::connect(url)
            .await
            .expect("Failed to connect to database")
    })
});

pub static PKG_NAME: &str = std::env!("CARGO_PKG_NAME");
