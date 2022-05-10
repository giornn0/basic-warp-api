use std::{convert::Infallible};

use http_api_problem::HttpApiProblem;
use warp::{reply::{Json},Rejection, hyper::StatusCode, Reply, reject::{MethodNotAllowed}, body::BodyDeserializeError};
use diesel::result::Error as DbError;
use crate::{services::errors::{QueryNotFound, InvalidParameter}, models::server::ApiResponse};

use super::{request::Error, errors::handling_db_errors};

pub fn response<T: serde::Serialize>(data: Result<T, DbError>)-> Result<Json,Rejection>{
  // let response = warp::reply::with_status(response,201);
  match data {
    Ok(success)=>{
      let response = warp::reply::json(&ApiResponse::<T>::from(success));
      Ok(response)
    },
    Err(e)=>{
      handling_db_errors(e)
    }
  }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
  println!("{:?}", err);
  let response = if let Some(e) = err.find::<Error>() {
      handle_crate_error(e)
  }else if let Some(e) = err.find::<BodyDeserializeError>(){
    let problem = HttpApiProblem::new(StatusCode::BAD_REQUEST)
      .title("Error while deserializing body")
      .detail(format!("Error => {:?}",e));
    problem
  } else if let Some(e) = err.find::<InvalidParameter>(){
    let problem = HttpApiProblem::new(StatusCode::BAD_REQUEST)
      .title("Invalid parameter")
      .detail(format!("Error => {:?}",e));
    problem
  }else if let Some(e) = err.find::<QueryNotFound>(){
    let problem = HttpApiProblem::new(StatusCode::NOT_FOUND)
        .title("Data not found")
        .detail(format!("Error => {:?}",e));
    problem
  }
  else if let Some(e) = err.find::<MethodNotAllowed>(){
    let problem = HttpApiProblem::new(StatusCode::METHOD_NOT_ALLOWED)
      .title("Error while running query")
      .detail(format!("Error => {:?}",e));
    problem
  } else {
      HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
          .detail(format!("{:?}",err))
  };

  Ok(response.to_hyper_response())
}

fn handle_crate_error(e: &Error) -> HttpApiProblem {
  match e {
      Error::Validation(errors) => {
          let mut problem =
              HttpApiProblem::new(StatusCode::BAD_REQUEST)
                  .title("One or more validation errors occurred")
                  .detail("Please refer to the errors property for additional details");

          let _ = problem.set_value("errors", errors.errors());

          problem
      }
  }
}