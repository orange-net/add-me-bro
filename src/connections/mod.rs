mod conn_svc;

use axum::{
    Router,
    routing::{delete, patch, post},
};

use crate::{
    connections::conn_svc::{create_conn_handler, delete_conn_handler, patch_conn_handler},
    routes::AppState,
};

pub fn get_connection_routes() -> Router<AppState> {
    Router::new()
        .route("/connections", post(create_conn_handler))
        .route("/connections/{conn_id}", delete(delete_conn_handler))
        .route("/connections/{conn_id}", patch(patch_conn_handler))
}
