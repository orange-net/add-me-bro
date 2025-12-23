mod conf;
mod connections;
mod db;
mod entity;
mod errors;
mod guards;
mod health;
mod routes;

use color_eyre::Result;

use crate::conf::ServiceConfig;
use crate::db::DBPoolBuilder;
use crate::routes::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let cfg = ServiceConfig::new()?;

    let pool = DBPoolBuilder::default()
        .host(cfg.database.host)
        .port(cfg.database.port)
        .username(cfg.database.username)
        .password(cfg.database.password)
        .db_name(cfg.database.database_name)
        .build()
        .await?;

    let app = routes::setup_routes().with_state(AppState::new(pool.db));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
