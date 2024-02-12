use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::config::init::{ConfigOverride, DownloadConfigOverride, ServeConfigOverride};

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(super) struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Number of concurrent downloads.
    /// It is recommended to set this less than 4, otherwise the download error may occur.
    /// Default is 1.
    #[arg(short, long)]
    pub concurrency: Option<usize>,

    /// Directory to save the downloaded files.
    /// Default is [`dirs::data_dir()`] + "/archivehost"
    #[arg(short, long)]
    pub root_dir: Option<PathBuf>,

    /// Port to serve the downloaded website and manager.
    /// Default is 3000.
    #[arg(short, long)]
    pub port: Option<u16>,
}

#[derive(Subcommand, Clone)]
pub(super) enum Commands {
    /// Download website from the Wayback Machine
    Download(DownloadArgs),
    /// Launch the website and manager
    Serve(ServeArgs),
    /// View config
    Config,
}

#[derive(Args, Clone)]
pub(super) struct DownloadArgs {
    pub url: String,
    /// Timestamp to search from
    #[arg(long)]
    pub from: Option<String>,
    /// Timestamp to search to
    #[arg(long)]
    pub to: Option<String>,
}

#[derive(Args, Clone)]
pub(super) struct ServeArgs {
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
}

impl From<Cli> for ConfigOverride {
    fn from(cli: Cli) -> Self {
        Self {
            root: cli.root_dir,
            download: DownloadConfigOverride {
                concurrency: cli.concurrency,
            },
            serve: ServeConfigOverride { port: cli.port },
        }
    }
}
