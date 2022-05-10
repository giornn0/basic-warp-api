use jsonwebtoken::{EncodingKey, DecodingKey};

pub static API: &str = "api";

pub fn get_env_key()->String{
  std::env::var("TOKEN_KEY")
      .unwrap_or("jaajkfsklñe".to_owned())
}

pub fn encoding_key()->EncodingKey{
  EncodingKey::from_secret(get_env_key().as_bytes())
}
pub fn decoding_key()->DecodingKey{
  DecodingKey::from_secret(get_env_key().as_bytes())
}