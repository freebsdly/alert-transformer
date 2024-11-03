use crate::commands::start::{start_server, StartServerArgs};
use clap::{Parser, Subcommand};
use config::Config;
use serde::Deserialize;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct AppCli {
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Start the server
    Start(StartServerArgs)
}

pub async fn exec() {
    let cli = AppCli::parse();

    match cli.command {
        Some(SubCommands::Start(start_server_args)) => {
            start_server(start_server_args).await.expect("Couldn't start server");
        }
        _ => {}
    }
}

pub fn parse_settings<'a, T: Deserialize<'a>>(path: &str) -> Result<T, anyhow::Error> {
    let settings = Config::builder()
        .add_source(config::File::with_name(path))
        .build()?;

    Ok(settings.try_deserialize::<T>()?)

}