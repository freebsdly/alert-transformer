use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct  AppCli {
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Start the server
    Start(StartArgs)
}

#[derive(Args)]
#[derive(Debug)]
pub struct StartArgs {
    /// Configuration file path
    #[arg(short, long, default_value = "alert-transformer.yaml")]
    pub(crate) path: String,
}