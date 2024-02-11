use std::path::{Path, PathBuf};

use once_cell::sync::OnceCell;

pub static DEFAULT_IGNORE_MIME_TYPES: &[&str] = &["application/zip"];

static DEFAULT_SAVE_PATH: &str = "./output";
pub static DATA_DIR: OnceCell<PathBuf> = OnceCell::new();
pub fn init_data_dir(dir: Option<String>) {
    let dir = dir.unwrap_or_else(|| DEFAULT_SAVE_PATH.to_string());
    let path = Path::new(&dir).to_path_buf();
    DATA_DIR.set(path).expect("Failed to set DATA_DIR");
}
