use diesel::result::Error;
use warp::{reject::Rejection, reply::Json};

use crate::models::{user::User, server::ApiResponse};

use super::errors::handling_db_errors;

pub fn response_user(data: Result<User, Error>) -> Result<Json, Rejection> {
    match data {
        Ok(success) => Ok(warp::reply::json(&ApiResponse::<User>::from(success.get_payload())))
        ,
        Err(e) => {
          handling_db_errors(e)
        }
    }
}
