use std::path::PathBuf;

use derivative::Derivative;
use normalize_path::NormalizePath;
use serde::{Deserialize, Serialize};

fn default_root() -> PathBuf {
    #[cfg(not(debug_assertions))]
    let dir = dirs::data_dir()
        .expect("Failed to get data dir")
        .join(crate::constant::PKG_NAME);

    #[cfg(debug_assertions)]
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../")
        .join("data");

    dir.normalize()
}

#[derive(Derivative)]
#[derivative(Default)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[derivative(Default(value = "default_root()"))]
    pub root: PathBuf,
    pub download: DownloadConfig,
    pub serve: ServeConfig,
}

#[derive(Derivative)]
#[derivative(Default)]
#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadConfig {
    #[derivative(Default(value = "1"))]
    pub concurrency: usize,
    #[derivative(Default(value = r#"vec!["application/zip".to_string()]"#))]
    pub ignored_mime_types: Vec<String>,
}

#[derive(Derivative)]
#[derivative(Default)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ServeConfig {
    #[derivative(Default(value = "3000"))]
    pub port: u16,
    pub host: Option<String>,
}

impl Config {
    pub fn download_dir(&self) -> PathBuf {
        self.root.join("downloads")
    }
}

impl ServeConfig {
    pub fn get_host(&self) -> String {
        self.host
            .clone()
            .unwrap_or_else(|| format!("localhost:{}", self.port))
    }
}
