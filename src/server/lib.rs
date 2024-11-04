use crate::api::root::{get_post, get_posts, root};
use crate::error::handle_error;
use crate::middleware::handler_404;
use axum::error_handling::{HandleError, HandleErrorLayer};
use axum::routing::get;
use axum::Router;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use anyhow::anyhow;
use tokio::net::TcpListener;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::log;
use entity::post::{Entity, Model};
use entity::prelude::Post;

pub mod error;
pub mod middleware;
pub mod api;
pub mod repository;



#[derive(Clone)]
pub struct AppState {
    api: Arc<ApiService>,
}

#[derive(Serialize, Clone)]
pub struct PostVo {
    pub id: i32,
    pub title: String,
    pub text: String,
}

impl Default for PostVo {
    fn default() -> Self {
        Self {
            id: 0,
            title: "".to_string(),
            text: "".to_string(),
        }
    }
}

impl From<Model> for PostVo {
    fn from(model: Model) -> Self {
        Self{
            id: model.id,
            title: model.title,
            text: model.text,
        }
    }
}

impl From<&Model> for PostVo {
    fn from(value: &Model) -> Self {
        Self{
            id: value.clone().id,
            title: value.clone().title,
            text: value.clone().text,
        }
    }
}

#[derive(Clone)]
pub struct ApiService {
    db_conn: Arc<DatabaseConnection>,
}

impl ApiService {

    pub async fn new(db_conn: Arc<DatabaseConnection>) -> Result<Self, anyhow::Error> {
        Ok(ApiService {
            db_conn,
        })
    }

    pub async fn get_post(&self, id: i32) -> Result<PostVo, anyhow::Error> {
        let result = Post::find_by_id(id).one(self.db_conn.as_ref()).await?;
        result.map(PostVo::from).ok_or_else(|| anyhow!("post not found"))
    }

    pub async fn get_posts(&self) -> Result<Vec<PostVo>, anyhow::Error> {
        let result = Post::find().all(self.db_conn.as_ref()).await?;
        Ok(result.iter().map(PostVo::from).collect())
    }
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct MessageProducerArgs {
    pub brokers: Vec<String>,
    pub topic: String,
}

pub struct MessageProducer {
    args: MessageProducerArgs,
}

impl MessageProducer {
    pub fn new(args: MessageProducerArgs) -> Self {
        MessageProducer {
            args
        }
    }

    pub fn send(&self, message: &str) {}
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct ApiArgs {
    pub host: String,
    pub port: u16,
    pub timeout: u64,
}


#[derive(Deserialize)]
#[derive(Clone)]
pub struct DatabaseArgs {
    #[serde(alias = "type")]
    db_type: String,
    #[serde(alias = "host")]
    db_host: String,
    #[serde(alias = "port")]
    db_port: u16,
    #[serde(alias = "name")]
    db_name: String,
    username: String,
    password: String,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct ServerArgs {
    #[serde(alias = "api")]
    pub api: ApiArgs,
    #[serde(alias = "database")]
    pub database: DatabaseArgs,
    #[serde(alias = "producer")]
    pub producer: MessageProducerArgs,
}

pub struct Server {
    api: Arc<ApiService>,
    producer: Arc<MessageProducer>,
    args: Arc<ServerArgs>,
    db_conn: Arc<DatabaseConnection>,
}

impl Server {
    pub async fn new(args: Arc<ServerArgs>) -> Result<Self, anyhow::Error> {
        // Connect to db
        let conn_str = format!("{}://{}:{}@{}:{}/{}",
                               args.database.db_type, args.database.username,
                               args.database.password, args.database.db_host,
                               args.database.db_port, args.database.db_name);
        let mut opt = ConnectOptions::new(conn_str);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("public"); // Setting default PostgreSQL schema
        let conn = Database::connect(opt).await?;
        conn.ping().await.expect("Failed to connect to database");
        let arc_conn = Arc::new(conn);
        let api = ApiService::new(arc_conn.clone()).await?;
        let producer = MessageProducer::new(args.producer.clone());
        Ok(Server {
            api: Arc::new(api),
            producer: Arc::new(producer),
            args,
            db_conn: arc_conn.clone(),
        })
    }

    pub async fn start(&self) -> Result<(), anyhow::Error> {
        // Create a regular axum app.
        let app = Router::new()
            .route("/", get(root))
            .route("/posts", get(get_posts))
            .route("/posts/:id", get(get_post))
            .fallback(handler_404)
            .layer((
                TraceLayer::new_for_http(),
                // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
                // requests don't hang forever.
                TimeoutLayer::new(Duration::from_secs(self.args.api.timeout)),
            ))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(handle_error))
                    .timeout(Duration::from_secs(self.args.api.timeout)),
            )
            .with_state(Arc::new(AppState{api: self.api.clone()}));

        // Create a `TcpListener` using tokio.
        let listener = TcpListener::bind(format!("{}:{}", self.args.api.host, self.args.api.port)).await?;

        tracing::debug!("listening on {}", listener.local_addr()?);
        // Run the server with graceful shutdown
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), anyhow::Error> {
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
