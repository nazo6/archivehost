use std::path::PathBuf;

mod interface;

pub use interface::*;

use normalize_path::NormalizePath;
use once_cell::sync::Lazy;

use crate::cli;

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

fn get_config(config_path: &PathBuf, or: ConfigOverride) -> eyre::Result<Config> {
    let mut config = confy::load_path::<Config>(config_path)?;

    merge!(config.root, or.root);
    merge!(config.download.concurrency, or.download_concurrency);
    merge!(config.serve.port, or.serve_port);
    merge_optional!(config.serve.host, or.serve_host);

    Ok(config)
}

pub static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    cli::CLI.config_path.clone().unwrap_or_else(|| {
        #[cfg(debug_assertions)]
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../")
            .join("data");
        #[cfg(not(debug_assertions))]
        let dir = dirs::config_dir()
            .expect("Failed to get config dir")
            .join(crate::constant::PKG_NAME);

        dir.normalize().join("config.toml")
    })
});

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let cli = cli::CLI.clone();

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

    let config = get_config(&CONFIG_PATH, config_override).expect("Failed to load config");

    if config.root.exists() && !config.root.is_dir() {
        panic!("Data dir is not a directory");
    }
    std::fs::create_dir_all(&*config.root).expect("Failed to create data dir");

    config
});
