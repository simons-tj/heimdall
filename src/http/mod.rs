pub mod error;
use std::sync::Arc;

use neo4rs::Graph;

mod handlers;
use handlers::health::health;
use handlers::relationship::create_relationship;

use crate::config::Config;
use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct AppState {
    config: Config,
    graph: Graph,
}

pub async fn serve(config: Config, graph: Graph) -> anyhow::Result<()> {
    let state = Arc::new(AppState { config, graph });
    let app = Router::new()
        .route("/health", get(health))
        .route("/relate", post(create_relationship))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
