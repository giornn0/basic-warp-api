use std::sync::Arc;

use warp::{Rejection, Reply, Filter};

use crate::{models::server::Pool, services::{server::with_pool, request::with_validated_json}, handlers::{auth::login, user::create_user}};

pub fn auth_router(db_pool: &Arc<Pool>)-> impl Filter<Extract= impl Reply, Error= Rejection> +Clone{
  let scope = warp::path("auth");
  let login_route = scope
      .and(warp::path("login"))
      .and(warp::post())
      .and(with_validated_json())
      .and(warp::path::end())
      .and(with_pool(db_pool.clone()))
      .and_then(login);
  let signup = scope
      .and(warp::path("signup"))
      .and(warp::post())
      .and(with_validated_json())
      .and(warp::path::end())
      .and(with_pool(db_pool.clone()))
      .and_then(create_user);
  login_route.or(signup)
}