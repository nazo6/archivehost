use std::path::PathBuf;

use super::{Config, CONFIG, DOWNLOAD_DIR, PKG_NAME};

pub struct ConfigOverride {
    pub root: Option<PathBuf>,
    pub download: DownloadConfigOverride,
    pub serve: ServeConfigOverride,
}

pub struct DownloadConfigOverride {
    pub concurrency: Option<usize>,
}

pub struct ServeConfigOverride {
    pub port: Option<u16>,
    pub host: Option<String>,
}

macro_rules! merge {
    ($path:ident, $dst:expr, $src:expr) => {
        if let Some(v) = $src.$path {
            $dst.$path = v;
        }
    };
}
macro_rules! merge_optional {
    ($path:ident, $dst:expr, $src:expr) => {
        $dst.$path = $src.$path;
    };
}

pub fn init_config(merge_config: ConfigOverride) {
    let mut config = confy::load::<Config>(PKG_NAME, None).expect("Failed to load config");

    merge!(root, config, merge_config);
    merge!(concurrency, config.download, merge_config.download);
    merge!(port, config.serve, merge_config.serve);
    merge_optional!(host, config.serve, merge_config.serve);

    CONFIG.set(config).expect("Failed to set config");
    DOWNLOAD_DIR
        .set(CONFIG.root.join("download"))
        .expect("Failed to set download dir");

    if CONFIG.root.exists() && !CONFIG.root.is_dir() {
        panic!("Data dir is not a directory");
    } else {
        std::fs::create_dir_all(&*CONFIG.root).expect("Failed to create data dir");
    }
    if DOWNLOAD_DIR.exists() && !DOWNLOAD_DIR.is_dir() {
        panic!("Download dir is not a directory");
    } else {
        std::fs::create_dir_all(&*DOWNLOAD_DIR).expect("Failed to create download dir");
    }
}
