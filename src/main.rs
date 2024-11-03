mod middleware;
mod error;
mod common;
mod repository;
mod api;
mod cli;
mod server;

use crate::cli::{AppCli, SubCommands};
use crate::server::run_server;
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = AppCli::parse();

    match cli.command {
        Some(SubCommands::Start(start_args)) => {
            run_server(start_args).await
        }
        _ => {}
    }
}