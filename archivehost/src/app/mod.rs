use crate::{
    cli::{Commands, CLI},
    config::CONFIG_PATH,
};

mod download;
mod fixdb;
mod serve;

pub async fn start() -> eyre::Result<()> {
    match CLI.command.clone() {
        Commands::Download(args) => {
            download::download(args).await?;
        }
        Commands::Serve(args) => {
            serve::serve(args).await?;
        }
        Commands::Config => {
            println!("Config location: {:?}", &*CONFIG_PATH);
            println!("Config content: {:#?}", &*crate::config::CONFIG);
        }
        Commands::FixDb => {
            fixdb::fixdb().await?;
        }
    }

    Ok(())
}
