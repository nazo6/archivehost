mod cli;
mod common;
mod config;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // .with_env_filter("archivehost=debug")
        .pretty()
        .init();

    cli::start().await?;

    Ok(())
}
