use crate::{models::{user::{User, UserBalance, UserPayload}, concept::Concept, server::ApiResponse}, services::errors::{Unauthorized, throw_error}};
use bigdecimal::BigDecimal;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error,
    PgConnection,
};
use warp::{reply::Json,Rejection};



pub fn set_balance(
    user_id: i32,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    value: BigDecimal,
) -> Result<User, Error> {
    use crate::schema::users::dsl::{id, users};
    diesel::update(users.filter(id.eq(user_id)))
        .set(&UserBalance::from(BigDecimal::from(value)))
        .get_result(conn)
}

pub fn manage_updates(
  result: Result<Concept, Error>,
  conn: &PooledConnection<ConnectionManager<PgConnection>>,
  log_user: UserPayload,
  previous: Concept,
) -> Result<Json, Rejection> {
  if let Ok(concept) = result {
      let value = BigDecimal::from(log_user
      .balance.unwrap_or(BigDecimal::from(0)) + concept.get_amount() - previous.get_amount());
      if let Ok(fixed_user) = set_balance(log_user.id, conn, value) {
          Ok(warp::reply::json(&ApiResponse::<User>::from(fixed_user)))
      } else {
          throw_error(Unauthorized::new())
      }
  } else {
      throw_error(Unauthorized::new())
  }
}
pub fn manage_create(
  result: Result<Concept, Error>,
  conn: &PooledConnection<ConnectionManager<PgConnection>>,
  log_user: UserPayload,
) -> Result<Json, Rejection> {
  if let Ok(concept) = result {
      let value = BigDecimal::from(log_user
      .balance.unwrap_or(BigDecimal::from(0)) + concept.get_amount());
      if let Ok(fixed_user) = set_balance(log_user.id, conn, value) {
          Ok(warp::reply::json(&ApiResponse::<User>::from(fixed_user)))
      } else {
          throw_error(Unauthorized::new())
      }
  } else {
      throw_error(Unauthorized::new())
  }
}