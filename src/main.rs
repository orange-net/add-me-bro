use axum::{Json, Router, http::Error, routing::get};
use color_eyre::Result;
use serde_json::{Value, json};

use crate::conf::ServiceConfig;

mod conf;

async fn root() -> &'static str {
    "Hello, AddMeBro!"
}

async fn health_check() -> Json<Value> {
    Json(json!({"status": "Healthy"}))
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let cfg = ServiceConfig::new()?;

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
