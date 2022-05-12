use std::sync::Arc;

use crate::{
    models::{server::{Pool, ApiResponse}, token::{LoginPayload, Token}, user::{User, UserPayloadLogged, UserPayload}},
    services::{errors::{throw_error, InvalidParameter, QueryNotFound, handling_db_errors}, response::response},
    schema::access_tokens,
};
use diesel::{prelude::*, result::Error, r2d2::{PooledConnection, ConnectionManager}};
use warp::{reply::Json, Rejection};

pub async fn login(payload: LoginPayload, db_pool: Arc<Pool>) -> Result<Json, Rejection> {
    use crate::schema::users::dsl::{email, users};
    let conn = db_pool.get().unwrap();
    let user_email: String = payload.get_email();
    let user_password: String = payload.get_pass();
    let user: Result<User, Error> = users.filter(email.eq(user_email)).get_result(&conn);
    if let Ok(user) = user {
        if user.check_password(user_password) {
            save_token(user, &conn)
        } else {
            throw_error(InvalidParameter::from("Incorrect password".to_owned()))
        }
    } else {
        throw_error(QueryNotFound::from("Email not registered".to_owned()))
    }
}

pub async fn logout(log_user: UserPayload, db_pool: Arc<Pool>)-> Result<Json, Rejection> {
  use crate::schema::access_tokens::dsl::{access_tokens, user_id};
  let conn = db_pool.get().unwrap();
  let result: Result<usize, Error>=diesel::delete(access_tokens.filter(user_id.eq(log_user.id))).execute(&conn);
  response(result)
}

pub fn save_token(user: User, conn: &PooledConnection<ConnectionManager<PgConnection>>)->Result<Json, Rejection>{
  if let Ok(token) = user.loging_in() {
    let result:Result<Token,Error> = diesel::insert_into(access_tokens::table)
        .values(&token)
        .get_result(conn);
    match result {
      Ok(_) =>Ok(warp::reply::json(&ApiResponse::<UserPayloadLogged>::from(user.payload_with_token(token.get_token())))),
      Err(error)=> handling_db_errors(error)
    }
} else {
    Err(warp::reject::reject())
}
}