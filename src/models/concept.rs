use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::schema::concepts;

use super::user::UserPayload;

#[derive(Queryable,Serialize)]
pub struct Concept{
  id: i32,
  name: String,
  amount: BigDecimal,
  image: Option<String>,
  user_id:i32,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime
}

impl Concept{
  pub fn get_id(self: &Concept)->i32{
    (*self).id
  }
  pub fn get_user_id(self: &Concept)->i32{
    (*self).user_id
  }
  pub fn get_amount(self: &Concept)->BigDecimal{
    (*self).amount.to_owned()
  }
}

#[derive(Debug,Insertable, AsChangeset, Deserialize)]
#[table_name = "concepts"]
pub struct NewConcept {
  name: String,
  image: Option<String>,
  amount: BigDecimal,
  user_id: i32
}

impl NewConcept{
  pub fn new(posted: PostConcept, user_payload: &UserPayload)->NewConcept{
    NewConcept{
      name: posted.name,
      image: posted.image,
      amount: posted.amount,
      user_id: (*user_payload).id
    }
  }
}

#[derive(Debug,Deserialize,Validate)]
pub struct PostConcept{
  #[validate(length(min = 1, max = 55))]
  name: String,
  image: Option<String>,
  amount: BigDecimal,
}