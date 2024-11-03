use clap::Args;
use tracing::info;
use server::{Server, ServerArgs};
use crate::cli::parse_settings;

#[derive(Args)]
#[derive(Debug)]
pub struct StartServerArgs {
    /// Configuration file path
    #[arg(short, long, default_value = "etc/alert-transformer.yaml")]
    pub path: String,
}

pub async fn start_server(args: StartServerArgs) -> Result<(), anyhow::Error> {
    info!("starting server using configuration: {:?}", args.path);
    let server_args = parse_settings::<ServerArgs>(&*args.path)?;
    let server = Server::new(&server_args);
    server.start().await
}