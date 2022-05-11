use std::sync::Arc;

use argon2::{
  password_hash::{
      rand_core::OsRng,
      PasswordHash, PasswordHasher, PasswordVerifier, SaltString
  },
  Argon2
};
use warp::{Filter,Rejection};

use crate::models::{user::UserPayload, token::Token, server::Pool};

use super::server::with_pool;

pub fn hash(value: String)-> Option<String> {
  let argon2 = Argon2::default();
  let salt = SaltString::generate(&mut OsRng);
  match argon2.hash_password(value.as_bytes(), &salt){
    Ok(hashed)=>Some(hashed.to_string()),
    Err(_)=>None
  }
}

pub fn check_hashed(hash: String, compare: String)->bool{
  let parsed_hash = PasswordHash::new(&hash).unwrap();
  Argon2::default().verify_password(compare.as_bytes(), &parsed_hash).is_ok()
}
pub fn with_authenticathed(db_pool: &Arc<Pool>)-> impl Filter<Extract=(UserPayload,), Error= Rejection>+ Clone
{
  warp::header::<String>("authorization").and(with_pool(db_pool.clone())).and_then(|token: String, db_pool: Arc<Pool>|async move{
    Token::with_db_functionality(token,db_pool).map_err(warp::reject::custom)
  })
}
// let password = b"hunter42"; // Bad password; don't actually use!