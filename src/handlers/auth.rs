use std::sync::Arc;

use crate::{
    models::{server::Pool, token::{LoginPayload, Token}, user::User},
    services::{errors::{throw_error, InvalidParameter, QueryNotFound}, response::response},
    schema::access_tokens,
};
use diesel::{prelude::*, result::Error};
use warp::{reply::Json, Rejection};

pub async fn login(payload: LoginPayload, db_pool: Arc<Pool>) -> Result<Json, Rejection> {
    use crate::schema::users::dsl::{email, users};
    let conn = db_pool.get().unwrap();
    let user_email: String = payload.get_email();
    let user_password: String = payload.get_pass();
    let user: Result<User, Error> = users.filter(email.eq(user_email)).get_result(&conn);
    if let Ok(user) = user {
        if user.check_password(user_password) {
            if let Ok(token) = user.loging_in() {
                let result:Result<Token,Error> = diesel::insert_into(access_tokens::table)
                    .values(&token)
                    .get_result(&conn);
                response(result)
            } else {
                Err(warp::reject::reject())
            }
        } else {
            throw_error(InvalidParameter::from("Incorrect password".to_owned()))
        }
    } else {
        throw_error(QueryNotFound::from("Email not registered".to_owned()))
    }
}
