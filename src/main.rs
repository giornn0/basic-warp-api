#[macro_use]
extern crate diesel;

mod app;
mod handlers;
mod models;
mod routes;
mod services;
mod schema;
mod utils;

use crate::services::server::start_db;
use app::app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::dotenv();

    let pool = start_db();

    app(&pool).await;

    Ok(())
}
