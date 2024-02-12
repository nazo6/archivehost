use crate::config::{cli::Commands, CLI};

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
            println!(
                "Config location: {:?}",
                confy::get_configuration_file_path(crate::config::PKG_NAME, None)?
            );
            println!("Current config: {:#?}", &*crate::config::CONFIG);
        }
        Commands::FixDb => {
            fixdb::fixdb().await?;
        }
    }

    Ok(())
}
