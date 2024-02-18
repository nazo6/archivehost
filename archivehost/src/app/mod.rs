use crate::{
    cli::{Commands, CLI},
    config::CONFIG_PATH,
};

mod download;
mod fixdb;
mod serve;

pub async fn start(conn: sea_orm::DatabaseConnection) -> eyre::Result<()> {
    match CLI.command.clone() {
        Commands::Download(args) => {
            download::download(&conn, args).await?;
        }
        Commands::Serve(args) => {
            serve::serve(conn, args).await?;
        }
        Commands::Config => {
            println!("Config location: {:?}", &*CONFIG_PATH);
            println!("Config content: {:#?}", &*crate::config::CONFIG);
        }
        Commands::FixDb => {
            fixdb::fixdb(&conn).await?;
        }
    }

    Ok(())
}
