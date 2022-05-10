use std::sync::Arc;

use jsonwebtoken::{decode, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use validator::Validate;
use chrono::NaiveDateTime;

use crate::{schema::access_tokens, utils::{dates::now_ndt, constants::decoding_key}, services::errors::Unauthorized};

use super::{user::UserPayload, server::Pool};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Token{
  id: i32,
  user_id: i32,
  token:String,
  valid_until:NaiveDateTime,
  created_at:NaiveDateTime,
  updated_at:NaiveDateTime
}

impl Token{
  pub fn get_user_payload(request_token: String, _db_pool: Arc<Pool> )->Result<UserPayload, Unauthorized>{
    if request_token.len() > 0{
      if let Ok(user)=decode::<UserPayload>( &request_token,&decoding_key(),&Validation::new(Algorithm::HS256)){
        Ok(user.claims)
      }else{
        Err(Unauthorized::new())
      }
    }else{
      Err(Unauthorized::from("Missing authorization token".to_owned()))
    }
  }

  // pub with_db_functionality(){
  //   use crate::schema::{access_tokens::dsl::{access_tokens,token,user_id}, users::dsl::users};
  //     let conn = db_pool.clone().get().unwrap();
  //     let finded_id: Option<i32> = access_tokens.select(user_id).filter(token.eq(request_token)).get_result(&conn).ok();
  //     if let  Some(checked_id) = finded_id{
  //         let user: Result<User,Error> = users.find(checked_id).get_result(&conn);
  //         if let Ok(finded_user) = user{
  //           Ok(finded_user.get_payload())
  //         }else{
  //           Err(Unauthorized::from("User not registered".to_owned()))
  //         }
  //     }else{
  //       Err(Unauthorized::from("User not logged".to_owned()))
  //     }
  // }
}


#[derive(Serialize,Deserialize,Debug, Insertable, AsChangeset)]
#[table_name = "access_tokens"]
pub struct NewToken{
  user_id: i32,
  token: String,
  valid_until:NaiveDateTime,
}
impl NewToken{
  pub fn from(token: String, user_id: i32)-> NewToken{
    NewToken{
      user_id,
      token,
      valid_until: now_ndt()
    }
  }
}
#[derive(Serialize,Deserialize, Debug, Validate)]
pub struct LoginPayload{
  #[validate(email)]
  email: String,
  #[validate(length(min = 2, max =255))]
  password: String,
}

impl LoginPayload{
  pub fn get_email(self: &LoginPayload)-> String{
    (*self.email).to_owned()
  }
  pub fn get_pass(self: &LoginPayload)-> String{
    (*self.password).to_owned()
  }
}
