use redis;
use redis::Commands;

use super::model;
use redis_context::RedisContext;
use std::time::SystemTime;
use uuid::Uuid;

/// Updates redis so that the individual measurement is applied to the correct tank.
/// Also records the measurement to a record associated with the sensor itself.
/// Keeps track of how many updates have been applied to each tank and sensor record.
/// Will create a new sensor record for this device if one does not already exist.
pub fn update(redis_ctx: &RedisContext, measure: &model::Measurement, ext_device_id: &str) {
    println!("Received redis {} update: {:?}", measure.name(), measure);
    let ext_device_namespace = &redis_ctx
        .get_external_device_namespace(measure.name())
        .unwrap();
    let device_id = internal_device_id(ext_device_id, ext_device_namespace).unwrap();

    println!("\tDevice ID (internal): {}", device_id);
    let rn = &redis_ctx.namespace;

    // add to the member set if it doesn't already exist
    let _ = redis::cmd("SADD")
        .arg(format!("{}/sensors/{}", rn, measure.name()))
        .arg(&format!("{}", device_id))
        .execute(&redis_ctx.conn);

    // lookup associated tank
    let sensor_hash_key = &format!("{}/sensors/{}/{}", rn, measure.name(), device_id).to_string();

    let tank_and_update_count: Result<Vec<Option<u64>>, _> = redis_ctx.conn.hget(
        sensor_hash_key,
        vec!["tank", &format!("{}_update_count", measure.name())],
    );

    if let Ok(v) = tank_and_update_count {
        if let Some(tank_num) = v.get(0).unwrap_or(&None) {
            update_tank_hash(redis_ctx, tank_num, &measure);
        } else {
            ensure_sensor_hash_exists(redis_ctx, sensor_hash_key, ext_device_id);
        };

        // record a hit on the updates that the sensor has seen
        // and also record the most recent measurement on the record
        // for this individual sensor
        let sensor_updated = update_sensor_hash(
            redis_ctx,
            sensor_hash_key,
            measure,
            v.get(1).unwrap_or(&None),
        );
        if let Err(e) = sensor_updated {
            println!("couldn't update sensor record {}: {:?}", sensor_hash_key, e);
        }
    };
}

fn update_tank_hash(redis_ctx: &RedisContext, tank_num: &u64, measure: &model::Measurement) {
    // We found the tank associated with this
    // sensor ID, so we should update that tank's
    // current reading.
    let tank_key = format!("{}/tanks/{}", redis_ctx.namespace, tank_num);

    let tank_measure_count: Result<Option<u32>, _> = redis_ctx
        .conn
        .hget(&tank_key, &format!("{}_update_count", measure.name()));

    let update: Result<String, _> = {
        let mut data: Vec<(&str, String)> = measure.to_redis();

        let uc_name = &format!("{}_update_count", measure.name());
        data.push((
            uc_name,
            tank_measure_count
                .unwrap_or(None)
                .map(|u| u + 1)
                .unwrap_or(1)
                .to_string(),
        ));

        let ut_name = &format!("{}_update_time", measure.name());
        data.push((ut_name, epoch_secs().to_string()));
        redis_ctx.conn.hset_multiple(&tank_key, &data[..])
    };

    if let Err(e) = update {
        println!("update fails for {}: {:?}", tank_key, e);
    }
}

fn ensure_sensor_hash_exists(
    redis_ctx: &RedisContext,
    sensor_hash_key: &str,
    ext_device_id_str: &str,
) {
    // We know that there's no associated "tank"
    // field for this key.  Let's make sure the record
    // for this sensor exists -- we'll need a human
    // to come in and link this device to a specific tank
    // using redis-cli!

    redis_ctx
        .conn
        .exists(sensor_hash_key)
        .iter()
        .for_each(|e: &bool| {
            if !e {
                // new sensor, make note of when it is created
                let _: Result<Vec<bool>, _> = redis_ctx.conn.hset_multiple(
                    sensor_hash_key,
                    &vec![
                        ("create_time", format!("{}", epoch_secs())),
                        ("ext_device_id", ext_device_id_str.to_string()),
                    ][..],
                );
            }
        });
}

fn update_sensor_hash(
    redis_ctx: &RedisContext,
    sensor_hash_key: &str,
    measure: &model::Measurement,
    maybe_sensor_upd_count: &Option<u64>,
) -> Result<(), redis::RedisError> {
    let upd_c = &format!("{}_update_count", measure.name());
    let mut data: Vec<(&str, String)> = vec![(
        upd_c,
        maybe_sensor_upd_count
            .map(|u| u + 1)
            .unwrap_or(1)
            .to_string(),
    )];
    data.extend(measure.to_redis());
    let ut = &format!("{}_update_time", measure.name());
    data.push((ut, epoch_secs().to_string()));

    redis_ctx.conn.hset_multiple(sensor_hash_key, &data[..])
}

fn internal_device_id(
    external_device_id: &str,
    external_device_namespace: &Uuid,
) -> Result<Uuid, uuid::parser::ParseError> {
    Ok(Uuid::new_v5(
        &external_device_namespace,
        external_device_id.as_bytes(),
    ))
}

fn epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}