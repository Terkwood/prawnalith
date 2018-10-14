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
