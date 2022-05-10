use std::num::ParseIntError;

pub fn port()-> Result<u16,ParseIntError>{
  std::env::var("PORT")
      .ok()
      .map(|val| val.parse::<u16>())
      .unwrap_or(Ok(8080))
}