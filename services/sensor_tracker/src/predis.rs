use tracker_support::epoch_secs;

use redis;
use redis::Commands;

use super::prawnqtt;
use redis_context::RedisContext;

pub fn receive_updates(
    update_r: std::sync::mpsc::Receiver<Option<paho_mqtt::message::Message>>,
    redis_ctx: &RedisContext,
    mqtt_cli: paho_mqtt::Client,
    measure_name: String,
) {
    loop {
        match update_r.try_recv() {
            Ok(Some(paho)) => {
                if let Some(measure) = prawnqtt::deser_message(paho) {
                    println!("\tReceived redis {} update: {:?}", measure_name, measure);
                    let device_id: String = format!(
                        "{}",
                        measure
                            .id(&redis_ctx
                                .get_external_device_namespace(measure_name.to_string())
                                .unwrap())
                            .unwrap()
                    );
                    println!("\tDevice ID (internal): {}", device_id);
                    let rn = &redis_ctx.namespace;

                    // add to the member set if it doesn't already exist
                    let _ = redis::cmd("SADD")
                        .arg(format!("{}/sensors/{}", rn, measure_name))
                        .arg(&device_id)
                        .execute(&redis_ctx.conn);

                    // lookup associated tank
                    let sensor_hash_key =
                        &format!("{}/sensors/{}/{}", rn, measure_name, device_id).to_string();

                    let assoc_tank_num: Result<Vec<Option<u64>>, _> = redis_ctx.conn.hget(
                        sensor_hash_key,
                        vec!["tank", &format!("{}_update_count", measure_name)],
                    );

                    let _ = assoc_tank_num.iter().for_each(|v| {
                        let maybe_tank_num = v.get(0).unwrap_or(&None);
                        let maybe_sensor_upd_count: &Option<_> = v.get(1).unwrap_or(&None);
                        if let Some(tank_num) = maybe_tank_num {
                            // We found the tank associated with this
                            // sensor ID, so we should update that tank's
                            // current reading.
                            let tank_key = format!("{}/tanks/{}", rn, tank_num);

                            let tank_measure_count: Result<
                                Option<u32>,
                                _,
                            > = redis_ctx
                                .conn
                                .hget(&tank_key, &format!("{}_update_count", measure_name));

                            let update: Result<String, _> = redis_ctx.conn.hset_multiple(
                                &tank_key,
                                &vec![
                                    ("temp_f", measure.temp_f.to_string()),
                                    ("temp_c", measure.temp_c.to_string()),
                                    (
                                        &format!("{}_update_time", measure_name),
                                        epoch_secs().to_string(),
                                    ),
                                    (
                                        &format!("{}_update_count", measure_name),
                                        tank_measure_count
                                            .unwrap_or(None)
                                            .map(|u| u + 1)
                                            .unwrap_or(1)
                                            .to_string(),
                                    ),
                                ][..],
                            );

                            if let Err(e) = update {
                                println!("update fails for {}: {:?}", tank_key, e);
                            }
                        } else {
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
                                        let _: Result<
                                            Vec<bool>,
                                            _,
                                        > = redis_ctx.conn.hset_multiple(
                                            sensor_hash_key,
                                            &vec![
                                                ("create_time", format!("{}", epoch_secs())),
                                                ("ext_device_id", measure.device_id.to_string()),
                                            ][..],
                                        );
                                    }
                                });
                        };

                        // record a hit on the updates that the sensor has seen
                        // and also record the most recent measurement on the record
                        // for this individual sensor
                        let update_sensor: Result<String, _> = redis_ctx.conn.hset_multiple(
                            sensor_hash_key,
                            &vec![
                                (
                                    &format!("{}_update_count", measure_name)[..],
                                    maybe_sensor_upd_count
                                        .map(|u| u + 1)
                                        .unwrap_or(1)
                                        .to_string(),
                                ),
                                ("temp_f", measure.temp_f.to_string()),
                                ("temp_c", measure.temp_c.to_string()),
                                (
                                    &format!("{}_update_time", measure_name),
                                    epoch_secs().to_string(),
                                ),
                            ][..],
                        );
                        if let Err(e) = update_sensor {
                            println!("couldn't update sensor record {}: {:?}", sensor_hash_key, e);
                        }
                    });
                    println!("");
                }
            }
            _ => {
                // Our MQTT abstraction has leaked into
                // our redis code.  This is unfortunate.
                // But without handling the reconnect case,
                // somehow the MQTT connection initially fails.
                // Too, we don't really trust the client to stay
                // connected indefinitely, so we'd like to continue
                // watching for this condition as long as
                // the program runs.
                if !mqtt_cli.is_connected() {
                    let _ = tracker_support::try_mqtt_reconnect(&mqtt_cli);
                }
            }
        }
    }
}
