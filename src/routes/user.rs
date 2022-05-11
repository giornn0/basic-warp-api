use std::sync::Arc;

use warp::{Filter, Reply, Rejection};

use crate::{services::{server::with_pool, request::with_validated_json, auth::with_authenticathed}, handlers::user::{get_user, create_user}, models::server::Pool, utils::constants::API};

pub fn users_router(db_pool: &Arc<Pool>)->impl Filter<Extract= impl Reply, Error= Rejection>+ Clone{
  let scope = warp::path(API).and(warp::path("users"));
  let get_one = scope
    .and(warp::get())
    .and(warp::path::param())
    .and(with_authenticathed(&db_pool))
    .and(warp::path::end())
    .and(with_pool(db_pool.clone()))
    .and_then(get_user);
  let create_one = scope
    .and(warp::post())
    .and(with_validated_json())
    .and(warp::path::end())
    .and(with_pool(db_pool.clone()))
    .and_then(create_user);
  get_one.or(create_one)
}