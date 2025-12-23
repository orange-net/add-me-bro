use axum::Router;
use sea_orm::DatabaseConnection;

use crate::{connections::get_connection_routes, health::setup_health_routes};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    without_validation_arguments: (),
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            without_validation_arguments: (),
        }
    }
}

pub fn setup_routes() -> Router<AppState> {
    Router::new()
        .merge(setup_health_routes())
        .merge(get_connection_routes())
}
