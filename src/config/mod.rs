use std::path::PathBuf;

pub mod cli;
mod interface;
use clap::Parser as _;
pub use interface::*;
use once_cell::sync::Lazy;
use sqlx::SqlitePool;

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

    if config.root.exists() && !config.root.is_dir() {
        panic!("Data dir is not a directory");
    } else {
        std::fs::create_dir_all(&*config.root).expect("Failed to create data dir");
    }

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

    get_config(config_override).unwrap()
});

pub static POOL: Lazy<SqlitePool> = Lazy::new(|| {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let pool = SqlitePool::connect(&CONFIG.root.join("db.sqlite?mode=rwc").to_string_lossy())
            .await
            .expect("Failed to connect to db");
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to migrate");
        pool
    })
});

pub static PKG_NAME: &str = std::env!("CARGO_PKG_NAME");
