use clap::Parser as _;

use crate::config::init::init_config;

use self::interface::{Cli, Commands};

mod download;
mod interface;
mod serve;

pub async fn start() -> eyre::Result<()> {
    let cli = Cli::parse();

    init_config(cli.clone().into());

    match cli.command {
        Commands::Download(args) => {
            download::download(args).await?;
        }
        Commands::Serve => {
            serve::serve().await?;
        }
        Commands::Config => {
            println!(
                "Config location: {:?}",
                confy::get_configuration_file_path(crate::config::PKG_NAME, None)?
            );
            println!("Current config: {:#?}", &*crate::config::CONFIG);
        }
    }

    Ok(())
}
