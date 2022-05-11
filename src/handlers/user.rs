use std::sync::Arc;

use crate::{
    models::{
        server::Pool,
        user::{NewUser, User, UserPayload},
    },
    schema::users,
    services::{errors::{InvalidParameter, handling_db_errors, throw_error, Unauthorized}, user_response::response_user}, handlers::auth::save_token,
};
use diesel::{prelude::*, result::Error};
use warp::{reply::Json, Rejection};

pub async fn get_user(id: i32,log_user: UserPayload, db_pool: Arc<Pool>) -> Result<Json, Rejection> {
    use crate::schema::users::dsl::users;
    if id == log_user.id{
      let conn = db_pool.get().unwrap();
      let result: Result<User, Error> = users.find(id).get_result(&conn);
      response_user(result)
    }else{
      throw_error(Unauthorized::new())
    }
}

pub async fn create_user(mut value: NewUser, db_pool: Arc<Pool>) -> Result<Json, Rejection> {
    use crate::schema::users::dsl::{email, users as user_dsl};
    let conn = db_pool.get().unwrap();
    let check_email = value.get_email();
    let unique_email: Option<String> = user_dsl
        .select(email)
        .filter(email.eq(check_email))
        .get_result(&conn)
        .ok();
    if let Some(used_email) = unique_email {
        Err(warp::reject::custom(InvalidParameter::from(format!(
            "The email {} is already in use",
            used_email
        ))))
    } else {
        let value = value.hash_password();
        let result: Result<User, Error> = diesel::insert_into(users::table)
            .values(value)
            .get_result(&conn);
        match result {
          Ok(user)=> save_token(user, &conn),
          Err(error)=> handling_db_errors(error)
        }
    }
}

// use std::sync::Arc;

// use warp::{Reply, Rejection, Filter, reply::Json};
// use diesel::prelude::*;

// use crate::{models::{Pool, Contacto, NewContacto, SearchQuery}, with_pool, response::response, schema::contactos};

// pub fn contactos_filter(db_pool: &Arc<Pool>)->impl Filter<Extract= impl Reply, Error= Rejection> + Clone{
//   let scope = warp::path("contactos");
//   let list = scope
//     .and(warp::get())
//     .and(warp::query())
//     .and(warp::path::end())
//     .and(with_pool(db_pool.clone()))
//     .and_then(get_contactos);
//   let get_one = scope
//     .and(warp::get())
//     .and(warp::path::param())
//     .and(with_pool(db_pool.clone()))
//     .and_then(get_contacto);
//   let create = scope
//     .and(warp::post())
//     .and(warp::body::json())
//     .and(with_pool(db_pool.clone()))
//     .and_then(create_contacto);
//   let update = scope
//     .and(warp::put())
//     .and(warp::path::param())
//     .and(warp::body::json())
//     .and(with_pool(db_pool.clone()))
//     .and_then(update_contacto);
//   let delete = scope
//     .and(warp::delete())
//     .and(warp::path::param())
//     .and(with_pool(db_pool.clone()))
//     .and_then(delete_contacto);
//   list.or(get_one).or(create).or(update).or(delete)
// }

// async fn get_contactos(query:SearchQuery,db_pool: Arc<Pool>)->Result<Json,Rejection>{
//   use crate::schema::contactos::dsl::contactos;
//   let conn = db_pool.get().unwrap();
//   let result:Vec<Contacto> = if let Some(take)= query._take{
//     contactos.limit(take).offset(if let Some(page)= query._page{(page-1)*take}else{0}).load::<Contacto>(&conn).expect("Error while retrieving all contactos!")
//   }else{
//     contactos.load::<Contacto>(&conn).expect("Error while retrieving all contactos!")
//   };
//   response(result)
// }
// async fn get_contacto(id:i32, db_pool: Arc<Pool>)->Result<Json,Rejection>{
//   use crate::schema::contactos::dsl::contactos;
//   let conn = db_pool.get().unwrap();
//   let result:Contacto = contactos.find(id).get_result(&conn).expect("Error while gettin contacto");
//   response(result)
// }
// async fn create_contacto(value: NewContacto, db_pool: Arc<Pool>)-> Result<Json,Rejection>{
//   let conn = db_pool.get().unwrap();
//   let result:Contacto = diesel::insert_into(contactos::table).values(value).get_result(&conn).expect("Error while trying to create contacto");
//   response(result)
// }
// async fn update_contacto(id:i32,value: NewContacto, db_pool: Arc<Pool>)-> Result<Json,Rejection>{
//   use crate::schema::contactos::dsl::contactos;
//   let conn = db_pool.get().unwrap();
//   let result:Contacto = diesel::update(contactos.find(id)).set(value).get_result(&conn).expect("Error while trying to update contacto!");
//   response(result)
// }
// async fn delete_contacto(delete_id:i32, db_pool: Arc<Pool>)-> Result<Json,Rejection>{
//   use crate::schema::contactos::dsl::{contactos, id};
//   let conn = db_pool.get().unwrap();
//   let result:usize = diesel::delete(contactos.filter(id.eq(delete_id))).execute(&conn).expect("Error while deleting contacto");
//   response(result)
// }
