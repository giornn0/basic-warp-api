use std::{sync::Arc, convert::Infallible};

use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use warp::{reply::Json,Rejection, Filter};

use crate::models::{server::Serving, server::Pool as DbPool};

pub async fn serve_start()->Result<Json,Rejection>{
  let api_service = warp::reply::json(&Serving{
      message:"Bienvenido, API con warp/postgres".to_string(),
      author:"giornn0".to_string()
  });
  Ok(api_service)
}
pub fn with_pool (db_pool: Arc<DbPool>)->impl Filter<Extract=(Arc<DbPool>,),Error= Infallible> + Clone{
  warp::any().map(move||db_pool.clone())
}

pub fn start_db()->Arc<DbPool>{
  let db_url = std::env::var("DATABASE_URL").expect("Missing database credentials!");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Arc::new(
        Pool::builder()
            .build(manager)
            .expect("Failed connection to database!"),
    )
}