/// All possible errors that this web app can throw
#[derive(Debug)]
pub enum WebError {
    RedisErr(redis::RedisError),
    ParseErr(uuid::parser::ParseError),
}

impl From<redis::RedisError> for WebError {
    fn from(error: redis::RedisError) -> Self {
        WebError::RedisErr(error)
    }
}

impl From<uuid::parser::ParseError> for WebError {
    fn from(error: uuid::parser::ParseError) -> Self {
        WebError::ParseErr(error)
    }
}
