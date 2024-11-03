use crate::api::root::root;
use crate::error::handle_error;
use crate::middleware::handler_404;
use axum::error_handling::HandleErrorLayer;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub mod error;
pub mod middleware;
pub mod common;
pub mod api;
pub mod repository;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct ApiServiceArgs {
    pub host: String,
    pub port: u16,
    pub timeout: u64,
}

pub struct ApiService<'a> {
    args: &'a ApiServiceArgs,
}

impl<'a> ApiService<'a> {
    pub fn new(args: &'a ApiServiceArgs) -> Self {
        ApiService {
            args,
        }
    }

    pub async fn start(&self) -> Result<(), anyhow::Error> {
        // Create a regular axum app.
        let app = Router::new()
            .route("/", get(root))
            .fallback(handler_404)
            .layer((
                TraceLayer::new_for_http(),
                // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
                // requests don't hang forever.
                TimeoutLayer::new(Duration::from_secs(self.args.timeout)),
            ))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(handle_error))
                    .timeout(Duration::from_secs(self.args.timeout)),
            );

        // Create a `TcpListener` using tokio.
        let listener = TcpListener::bind(format!("{}:{}", self.args.host, self.args.port)).await?;

        tracing::debug!("listening on {}", listener.local_addr()?);
        // Run the server with graceful shutdown
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct MessageProducerArgs {
    pub brokers: Vec<String>,
    pub topic: String,
}

pub struct MessageProducer<'a> {
    args: &'a MessageProducerArgs,
}

impl<'a> MessageProducer<'a> {
    pub fn new(args: &'a MessageProducerArgs) -> Self {
        MessageProducer {
            args
        }
    }

    pub fn send(&self, message: &str) {}
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct ServerArgs {
    #[serde(alias = "api")]
    pub api_args: ApiServiceArgs,
    #[serde(alias = "kafka")]
    pub producer_args: MessageProducerArgs,
}

pub struct Server<'a> {
    api: ApiService<'a>,
    msg_producer: MessageProducer<'a>,
    server_args: &'a ServerArgs,
}

impl<'a> Server<'a> {
    pub fn new(args: &'a ServerArgs) -> Self {
        Server {
            api: ApiService::new(&args.api_args),
            msg_producer: MessageProducer::new(&args.producer_args),
            server_args: args,
        }
    }

    pub async fn start(&self) -> Result<(), anyhow::Error> {
        self.api.start().await?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}