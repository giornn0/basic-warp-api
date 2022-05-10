use diesel::{r2d2::{ConnectionManager, self}, PgConnection, result::Error};
use serde::{Serialize, Deserialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize, Clone)]
pub struct Serving{
    pub message:String,
    pub author:String
}

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub _page: Option<i64>,
    pub _take: Option<i64>,
    pub _search: Option<String>
}

#[derive(Serialize,Deserialize)]
pub struct ApiResponse<T>{
  data: T,
  pagination: Option<Pagination>,
}
impl <T>ApiResponse<T>{
  pub fn new<G: serde::Serialize>(result: Result<G, Error>)->Result<ApiResponse<G>, Error>{
    match result {
      Ok(data)=>Ok(ApiResponse{
        data,
        pagination: None
      }),
      Err(error)=>Err(error)
    }
  }
  pub fn from<G: serde::Serialize>(data: G)-> ApiResponse<G>{
    ApiResponse{data, pagination: None}
  }
}
#[derive(Serialize,Deserialize)]
pub struct Pagination{
  page: i32,
  total_pages: i32,
  items_per_page: i32
}