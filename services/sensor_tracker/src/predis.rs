use redis::Commands;

use super::model;
use redis_context::RedisContext;
use redis_delta::REvent;
use serde_json;
use std::time::SystemTime;
use uuid::Uuid;

/// Updates redis so that the individual measurement is applied to the correct tank.
/// Also records the measurement to a record associated with the sensor itself.
/// Keeps track of how many updates have been applied to each tank and sensor record.
/// Will create a new sensor record for this device if one does not already exist.
pub fn update<'a, 'b>(
    redis_ctx: &RedisContext,
    sensor_message: &model::SensorMessage,
    ext_device_id: &str,
) -> Result<Vec<REvent>, redis::RedisError> {
    let mut delta_events: Vec<REvent> = vec![];

    println!("Received redis update: {:?}", sensor_message);

    let ext_device_namespace = &redis_ctx.get_external_device_namespace()?;
    let device_id = internal_device_id(ext_device_id, ext_device_namespace).unwrap();

    println!("\tDevice ID (internal): {}", device_id);
    let rn = &redis_ctx.namespace;

    let sensor_set_event = update_devices_set(redis_ctx, rn, sensor_message, device_id);
    if let Some(e) = sensor_set_event {
        delta_events.push(e)
    }

    // lookup associated area
    let sensor_hash_key = &format!("{}/devices/{}", rn, device_id).to_string();

    let area_and_sensors_update_count: Result<Vec<Option<u64>>, _> = redis_ctx
        .conn
        .hget(sensor_hash_key, vec!["area", "sensors_update_count"]);

    if let Ok(v) = area_and_sensors_update_count {
        // Tank associated with this sensor?
        let revent = match v.get(0).unwrap_or(&None) {
            Some(area_num) => update_area_hash(redis_ctx, area_num, &sensor_message),
            None => ensure_device_hash_exists(redis_ctx, sensor_hash_key, ext_device_id),
        };

        if let Some(ev) = revent {
            delta_events.push(ev)
        }

        // record a hit on the updates that the sensor has seen
        // and also record the most recent measurement on the record
        // for this individual sensor
        let sensor_updated = update_device_hash(
            redis_ctx,
            sensor_hash_key,
            sensor_message,
            v.get(2).unwrap_or(&None),
        );

        if let Some(ev) = sensor_updated {
            delta_events.push(ev)
        }
    };

    Ok(delta_events)
}

fn update_devices_set(
    redis_ctx: &RedisContext,
    rn: &str,
    sensor_message: &model::SensorMessage, // TODO wat ?
    device_id: Uuid,
) -> Option<REvent> {
    let set_device_key = format!("{}/devices", rn);
    // add to the member set if it doesn't already exist
    let devices_added: Result<u64, _> = redis_ctx
        .conn
        .sadd(&set_device_key, &format!("{}", device_id));

    match devices_added {
        Ok(n) if n > 0 => Some(REvent::SetUpdated {
            key: set_device_key,
        }),
        _ => None,
    }
}

fn update_area_hash(
    redis_ctx: &RedisContext,
    container_num: &u64,
    sensor_message: &model::SensorMessage,
) -> Option<REvent> {
    // We found the area associated with this
    // sensor ID, so we should update that area's
    // current reading.
    let area_key = format!("{}/areas/{}", redis_ctx.namespace, container_num);

    let area_measure_count: Result<Option<u32>, _> = redis_ctx
        .conn
        .hget(&area_key, &format!("sensors_update_count"));

    let uc_name = format!("sensors_update_count");
    let ut_name = format!("sensors_update_time");
    let update: (Result<String, _>, Vec<&str>) = {
        let mut data: Vec<(&str, String)> = sensor_message.to_redis();

        data.push((
            &uc_name,
            area_measure_count
                .unwrap_or(None)
                .map(|u| u + 1)
                .unwrap_or(1)
                .to_string(),
        ));

        data.push((&ut_name, epoch_secs().to_string()));
        (
            redis_ctx.conn.hset_multiple(&area_key, &data[..]),
            data.iter().map(|(a, _)| *a).collect(),
        )
    };

    match update {
        (Err(e), _) => {
            println!("update fails for {}: {:?}", area_key, e);
            None
        }
        (Ok(_), fields) if fields.len() > 0 => {
            let fs = fields.iter().map(|s| s.to_string()).collect();
            Some(REvent::HashUpdated {
                key: area_key.to_string(),
                fields: fs,
            })
        }
        _ => None,
    }
}

fn ensure_device_hash_exists(
    redis_ctx: &RedisContext,
    device_hash_key: &str,
    ext_device_id_str: &str,
) -> Option<REvent> {
    // We know that there's no associated "tank"
    // field for this key.  Let's make sure the record
    // for this sensor exists -- we'll need a human
    // to come in and link this device to a specific tank
    // using redis-cli!
    let mut result: Option<REvent> = None;

    redis_ctx
        .conn
        .exists(device_hash_key)
        .iter()
        .for_each(|e: &bool| {
            if !e {
                let cf = "create_time".to_string();
                let ed = "ext_device_id".to_string();
                let field_vals = &vec![
                    (&cf, format!("{}", epoch_secs())),
                    (&ed, ext_device_id_str.to_string()),
                ][..];
                // new sensor, make note of when it is created
                let _: Result<Vec<bool>, _> =
                    redis_ctx.conn.hset_multiple(device_hash_key, field_vals);

                let fields = vec![cf, ed];
                result = Some(REvent::HashUpdated {
                    key: device_hash_key.to_string(),
                    fields,
                })
            }
        });

    result
}

fn update_device_hash(
    redis_ctx: &RedisContext,
    device_hash_key: &str,
    sensor_message: &model::SensorMessage,
    maybe_sensor_upd_count: &Option<u64>,
) -> Option<REvent> {
    let upd_c = &format!("sensors_update_count");
    let mut data: Vec<(&str, String)> = vec![(
        upd_c,
        maybe_sensor_upd_count
            .map(|u| u + 1)
            .unwrap_or(1)
            .to_string(),
    )];
    data.extend(sensor_message.to_redis());
    let ut = &format!("sensors_update_time");
    data.push((ut, epoch_secs().to_string()));

    let redis_result: Result<(), _> = redis_ctx.conn.hset_multiple(device_hash_key, &data[..]);
    if let Err(e) = redis_result {
        println!("couldn't update device record {}: {:?}", device_hash_key, e);
        None
    } else {
        let mut fields: Vec<String> = vec![];
        data.iter().for_each(|(f, _)| fields.push(f.to_string()));

        Some(REvent::HashUpdated {
            key: device_hash_key.to_string(),
            fields,
        })
    }
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

pub fn publish_updates(redis_ctx: &RedisContext, topic: &str, updates: Vec<REvent>) {
    updates.iter().for_each(|delta_event| {
        if let Ok(s) = serde_json::to_string(delta_event) {
            let published: Result<u64, _> = redis_ctx.conn.publish(topic, s);
            if let Err(e) = published {
                println!("Error publishing to {}: {}", topic, e)
            }
        }
    })
}
