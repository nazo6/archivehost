use std::path::PathBuf;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

use super::PKG_NAME;

#[derive(Derivative)]
#[derivative(Default)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[derivative(Default(
        value = r#"dirs::data_dir().expect("Failed to get data dir").join(PKG_NAME)"#
    ))]
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
