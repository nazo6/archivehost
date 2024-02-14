use std::path::PathBuf;

pub mod cli;
mod interface;

pub use interface::*;

use clap::Parser as _;
use once_cell::sync::Lazy;
use sea_orm::{Database, DatabaseConnection};

pub struct ConfigOverride {
    pub root: Option<PathBuf>,
    pub download_concurrency: Option<usize>,
    pub serve_port: Option<u16>,
    pub serve_host: Option<String>,
}

macro_rules! merge {
    ($dst:expr, $src:expr) => {
        if let Some(v) = $src {
            $dst = v;
        }
    };
}
macro_rules! merge_optional {
    ($dst:expr, $src:expr) => {
        $dst = $src;
    };
}

fn get_config(or: ConfigOverride) -> eyre::Result<Config> {
    let mut config = confy::load::<Config>(PKG_NAME, None)?;

    merge!(config.root, or.root);
    merge!(config.download.concurrency, or.download_concurrency);
    merge!(config.serve.port, or.serve_port);
    merge_optional!(config.serve.host, or.serve_host);

    Ok(config)
}

pub static CLI: Lazy<cli::Cli> = Lazy::new(cli::Cli::parse);

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let cli = CLI.clone();
    let mut config_override = ConfigOverride {
        root: cli.root_dir,
        download_concurrency: None,
        serve_port: None,
        serve_host: None,
    };

    match cli.command {
        cli::Commands::Download(args) => {
            config_override.download_concurrency = args.concurrency;
        }
        cli::Commands::Serve(args) => {
            config_override.serve_port = args.port;
            config_override.serve_host = args.host;
        }
        _ => {}
    }

    let config = get_config(config_override).expect("Failed to load config");

    if config.root.exists() && !config.root.is_dir() {
        panic!("Data dir is not a directory");
    }
    std::fs::create_dir_all(&*config.root).expect("Failed to create data dir");

    config
});

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
