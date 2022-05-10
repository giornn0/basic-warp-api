use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use crate::{
    handlers::concept::{
        all_concepts, create_concept, get_concept, remove_concept, update_concept,
    },
    models::server::Pool,
    services::{auth::with_authenticathed, request::with_validated_json, server::with_pool},
    utils::constants::API,
};

pub fn concepts_router(
    db_pool: &Arc<Pool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let scope = warp::path(API)
        .and(warp::path("concepts"));
    let list = scope
        .and(warp::get())
        .and(warp::query())
        .and(with_authenticathed(&db_pool))
        .and(warp::path::end())
        .and(with_pool(db_pool.clone()))
        .and_then(all_concepts);
    let create = scope
        .and(warp::post())
        .and(with_authenticathed(&db_pool))
        .and(with_validated_json())
        .and(with_pool(db_pool.clone()))
        .and(warp::path::end())
        .and_then(create_concept);
    let delete = scope
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_authenticathed(&db_pool))
        .and(warp::path::end())
        .and(with_pool(db_pool.clone()))
        .and_then(remove_concept);
    let update = scope
        .and(warp::put())
        .and(warp::path::param())
        .and(with_authenticathed(&db_pool))
        .and(with_validated_json())
        .and(with_pool(db_pool.clone()))
        .and(warp::path::end())
        .and_then(update_concept);
    let get_one = scope
        .and(warp::get())
        .and(warp::path::param())
        .and(with_authenticathed(&db_pool))
        .and(warp::path::end())
        .and(with_pool(db_pool.clone()))
        .and_then(get_concept);
    list.or(create).or(delete).or(update).or(get_one)
}
