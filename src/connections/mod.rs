mod conn_svc;

use axum::{
    Router,
    routing::{delete, post},
};

use crate::{
    connections::conn_svc::{create_conn_handler, delete_conn_handler},
    routes::AppState,
};

pub fn get_connection_routes() -> Router<AppState> {
    Router::new()
        .route("/connections", post(create_conn_handler))
        .route("/connections", delete(delete_conn_handler))
}
