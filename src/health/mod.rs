use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

use crate::routes::AppState;

async fn health_check() -> Json<Value> {
    Json(json!({"status": "Healthy"}))
}

pub fn setup_health_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
}
