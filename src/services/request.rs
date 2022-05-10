use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};
use warp::{Filter, Rejection};

#[derive(Debug)]
pub enum Error {
    Validation(ValidationErrors),
    // Authorization()
}
impl warp::reject::Reject for Error {}

pub fn validate<T>(value: T) -> Result<T, Error>
where
    T: Validate,
{
    value.validate().map_err(Error::Validation)?;

    Ok(value)
}

pub fn with_validated_json<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: DeserializeOwned + Validate + Send,
{
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
        .and_then(|value| async move { validate(value).map_err(warp::reject::custom) })
}

