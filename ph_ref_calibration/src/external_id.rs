use uuid::Uuid;

pub fn resolve(
    external_id: &str,
    external_device_namespace: Uuid,
) -> Result<Uuid, uuid::parser::ParseError> {
    Ok(Uuid::new_v5(
        &external_device_namespace,
        external_id.as_bytes(),
    ))
}

#[derive(Debug)]
pub enum ResolveError {
    RedisErr(redis::RedisError),
    ParseErr(uuid::parser::ParseError),
}

impl From<redis::RedisError> for ResolveError {
    fn from(error: redis::RedisError) -> Self {
        ResolveError::RedisErr(error)
    }
}

impl From<uuid::parser::ParseError> for ResolveError {
    fn from(error: uuid::parser::ParseError) -> Self {
        ResolveError::ParseErr(error)
    }
}
