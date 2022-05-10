use bigdecimal::BigDecimal;
use jsonwebtoken::{encode, errors::Error, Header};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    schema::users,
    services::auth::{check_hashed, hash},
    utils::constants::encoding_key,
};

use super::token::NewToken;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    name: String,
    lastname: String,
    state: Option<bool>,
    password: String,
    email: String,
    balance: Option<BigDecimal>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserPayload {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub state: Option<bool>,
    pub balance: Option<BigDecimal>,
}

impl User {
    pub fn check_password(self: &User, compare: String) -> bool {
        check_hashed((*self.password).to_owned(), compare)
    }
    pub fn get_payload(self: &User) -> UserPayload {
        UserPayload {
            id: (*self).id,
            name: (*self.name).to_owned(),
            lastname: (*self.lastname).to_owned(),
            state: (*self).state,
            balance: (*self).balance.to_owned(),
        }
    }
    pub fn loging_in(self: &User) -> Result<NewToken, Error> {
        match encode(&Header::default(), &self.get_payload(), &encoding_key()) {
            Ok(token) => Ok(NewToken::from(token, self.id)),
            Err(error) => Err(error),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Insertable, AsChangeset, Validate)]
#[table_name = "users"]
pub struct NewUser {
    #[validate(length(min = 2, max = 55))]
    name: String,
    #[validate(length(min = 2, max = 55))]
    lastname: String,
    state: bool,
    #[validate(email)]
    email: String,
    password: String,
}
impl NewUser {
    pub fn hash_password(self: &mut NewUser) -> &NewUser {
        match hash((*self.password).to_string()) {
            Some(hashed) => {
                self.password = hashed;
                self
            }
            _ => self,
        }
    }
    pub fn get_email(self: &NewUser) -> String {
        self.email.to_owned()
    }
}
