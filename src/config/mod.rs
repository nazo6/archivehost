use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use self::once_cell_wrapper::OnceCellWrapper;

pub mod init;
mod once_cell_wrapper;

pub static PKG_NAME: &str = std::env!("CARGO_PKG_NAME");

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub root: PathBuf,
    pub download: DownloadConfig,
    pub serve: ServeConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadConfig {
    pub concurrency: usize,
    pub ignored_mime_types: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServeConfig {
    pub port: u16,
    pub host: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root: dirs::data_dir()
                .expect("Failed to get data dir")
                .join(PKG_NAME),
            download: DownloadConfig::default(),
            serve: ServeConfig::default(),
        }
    }
}
impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            concurrency: 1,
            ignored_mime_types: ["application/zip"].iter().map(|v| v.to_string()).collect(),
        }
    }
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            host: None,
        }
    }
}

impl Config {
    pub fn get_host(&self) -> String {
        self.serve
            .host
            .clone()
            .unwrap_or_else(|| format!("localhost:{}", self.serve.port))
    }
}

pub static CONFIG: OnceCellWrapper<Config> = OnceCellWrapper::new();

pub static DOWNLOAD_DIR: OnceCellWrapper<PathBuf> = OnceCellWrapper::new();
