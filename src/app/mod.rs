use clap::{Args, Parser, Subcommand};

use crate::config::init_data_dir;

mod download;
mod serve;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    dir: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Download website from the Wayback Machine
    #[command(short_flag = 'd')]
    Download(DownloadArgs),
    /// Serve the downloaded website
    #[command(short_flag = 's')]
    Serve(ServeArgs),
}

#[derive(Args)]
struct DownloadArgs {
    url: String,
    /// Maximum number of concurrent downloads.
    /// It is recommended to keep this value lower than 4. Otherwise, you may get rate-limited.
    #[arg(short, long, default_value_t = 1)]
    concurrency: u8,
    /// Timestamp to search from
    #[arg(long)]
    from: Option<String>,
    /// Timestamp to search to
    #[arg(long)]
    to: Option<String>,
}

#[derive(Args)]
struct ServeArgs {
    // host: String,
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

pub async fn start() -> eyre::Result<()> {
    let cli = Cli::parse();

    init_data_dir(cli.dir);

    match cli.command {
        Commands::Download(args) => {
            download::download(args).await?;
        }
        Commands::Serve(args) => {
            serve::serve(args).await?;
        }
    }

    Ok(())
}
