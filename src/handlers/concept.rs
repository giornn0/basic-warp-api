use std::sync::Arc;

use diesel::{prelude::*, result::Error};
use warp::{reply::Json, Rejection};

use crate::{
    models::{
        concept::{Concept, NewConcept, PostConcept},
        server::{Pool, SearchQuery},
        user::UserPayload,
    },
    schema::concepts,
    services::{
        errors::{throw_error, QueryNotFound, Unauthorized},
        response::response,
    },
};

pub async fn all_concepts(
    query: SearchQuery,
    log_user: UserPayload,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::concepts::dsl::concepts;
    let conn = db_pool.get().unwrap();
    let result = if let Some(take) = query._take {
        concepts
            .limit(take)
            .offset(if let Some(page) = query._page {
                (page - 1) * take
            } else {
                0
            })
            .load::<Concept>(&conn)
    } else {
        concepts.load::<Concept>(&conn)
    };
    response(result)
}

pub async fn get_concept(
    id: i32,
    log_user: UserPayload,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::concepts::dsl::concepts;
    let conn = db_pool.get().unwrap();
    let result: Result<Concept, Error> = concepts.find(id).get_result(&conn);
    match result {
        Ok(concept) => {
            if concept.get_id() == log_user.id {
                Ok(warp::reply::json(&concept))
            } else {
                throw_error(Unauthorized::new())
            }
        }
        _ => throw_error(QueryNotFound::new()),
    }
}

pub async fn create_concept(
    log_user: UserPayload,
    value: PostConcept,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    let conn = db_pool.get().unwrap();
    let inserted = NewConcept::new(value, &log_user);
    let result: Result<Concept, Error> = diesel::insert_into(concepts::table)
        .values(inserted)
        .get_result(&conn);
    response(result)
}
pub async fn remove_concept(
    delete_id: i32,
    log_user: UserPayload,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::concepts::dsl::{concepts, id};
    if delete_id == log_user.id {
        let conn = db_pool.get().unwrap();
        let result: Result<usize, Error> =
            diesel::delete(concepts.filter(id.eq(delete_id))).execute(&conn);
        response(result)
    } else {
        throw_error(Unauthorized::new())
    }
}

pub async fn update_concept(
    update_id: i32,
    log_user: UserPayload,
    value: PostConcept,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::concepts::dsl::{concepts, id, user_id};
    let conn = db_pool.get().unwrap();
    let updated = NewConcept::new(value, &log_user);
    let result: Result<Concept, Error> =
        diesel::update(concepts.filter(id.eq(update_id)).filter(user_id.eq(log_user.id)))
            .set(updated)
            .get_result(&conn);
    response(result)
}
