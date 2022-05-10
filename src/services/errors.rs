use warp::{reject, Rejection, reply::Json};
use diesel::result::Error;

#[derive(Debug)]
pub struct InvalidParameter{
  _message: String,
}
impl reject::Reject for InvalidParameter {}

impl InvalidParameter {
  pub fn from(_message: String)-> InvalidParameter{
    InvalidParameter{
      _message
    }
  }
}

#[derive(Debug)]
pub struct QueryNotFound{
  _message: String,
}
impl reject::Reject for QueryNotFound {}

impl QueryNotFound {
  pub fn new()->QueryNotFound{
    QueryNotFound{
      _message: "Empty query result".to_owned()
    }
  }
  pub fn from(_message: String)-> QueryNotFound{
    QueryNotFound{
      _message
    }
  }
}
#[derive(Debug)]
pub struct Unauthorized{
  _message: String,
}
impl reject::Reject for Unauthorized {}

impl Unauthorized {
  pub fn new()->Unauthorized{
    Unauthorized { _message: "Permision denied to perform this action".to_owned() }
  }
  pub fn from(_message: String)-> Unauthorized{
    Unauthorized{
      _message
    }
  }
}

pub fn throw_error<T: warp::reject::Reject>(error: T)-> Result<Json,Rejection>{
  Err(warp::reject::custom(error))
}

pub fn handling_db_errors(e: Error)->Result<Json,Rejection>{
  println!("Error => {:?}", e);
      match e{
        Error::NotFound => Err(warp::reject::custom(QueryNotFound::from(format!("No data available. Error => {}",e)))),
        _ => Err(warp::reject::reject()),
      }
}