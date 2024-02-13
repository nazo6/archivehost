use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Directory to save the downloaded files.
    /// Default is [`dirs::data_dir()`] + "/archivehost"
    #[arg(short, long)]
    pub root_dir: Option<PathBuf>,

    /// Skip migration.
    /// Warning: This is for development. Using this in production may cause data loss.
    #[arg(long, default_value_t = false)]
    pub skip_migration: bool,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Download website from the Wayback Machine
    Download(DownloadArgs),

    /// Launch the website and manager
    Serve(ServeArgs),

    /// Remove database entry that is not found in the filesystem.
    FixDb,

    /// View config.
    /// Config load strategy:
    ///   1. Loaded from the config file. (Usually in $XDG_DATA_HOME/archivehost)
    ///   2. Fallback to the default value.
    ///   3. Overrided with the CLI arguments.
    #[clap(verbatim_doc_comment)]
    Config,
}

#[derive(Args, Clone)]
pub struct DownloadArgs {
    pub url: String,

    /// Timestamp to search from
    #[arg(long)]
    pub from: Option<String>,

    /// Timestamp to search to
    #[arg(long)]
    pub to: Option<String>,

    /// Number of concurrent downloads.
    /// It is recommended to set this less than 4, otherwise the download error may occur.
    /// Default is 1.
    #[arg(short, long)]
    pub concurrency: Option<usize>,
}

#[derive(Args, Clone)]
pub struct ServeArgs {
    /// Port to serve the downloaded website and manager.
    /// Default is 3000.
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Hostname to serve the downloaded website and manager.
    /// This is only used to rewrite links using this value when serving the downloaded site.
    /// Default is "localhost:{port}"
    #[clap(verbatim_doc_comment)]
    #[arg(long)]
    pub host: Option<String>,
}
