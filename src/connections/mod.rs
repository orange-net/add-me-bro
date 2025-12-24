mod conn_svc;

use axum::{Router, routing::post};

use crate::{connections::conn_svc::create_conn_handler, routes::AppState};

pub fn get_connection_routes() -> Router<AppState> {
    Router::new().route("/connections", post(create_conn_handler))
    Router::new().route("/connections", delete())
}
