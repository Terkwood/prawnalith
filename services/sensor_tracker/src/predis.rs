use redis;
use redis::Commands;

use super::model;
use super::prawnqtt;
use redis_context::RedisContext;
use std::time::SystemTime;

pub fn receive_updates(
    update_r: std::sync::mpsc::Receiver<Option<paho_mqtt::message::Message>>,
    redis_ctx: &RedisContext,
    mqtt_cli: paho_mqtt::Client,
) {
    loop {
        match update_r.try_recv() {
            Ok(Some(paho)) => {
                if let Some(sensor_message) = prawnqtt::deser_message(paho) {
                    let ext_id_str: &str = &sensor_message.device_id;

                    sensor_message.measurements().iter().for_each(|measure| {
                        println!("\tReceived redis {} update: {:?}", measure.name(), measure);
                        let ext_device_namespace = &redis_ctx
                            .get_external_device_namespace(measure.name())
                            .unwrap();
                        let ext_id = &mut model::ExternalDeviceId {
                            external_id: ext_id_str.to_string(),
                        };
                        let device_id = ext_id.to_internal_id(ext_device_namespace).unwrap();

                        println!("\tDevice ID (internal): {}", device_id);
                        let rn = &redis_ctx.namespace;

                        // add to the member set if it doesn't already exist
                        let _ = redis::cmd("SADD")
                            .arg(format!("{}/sensors/{}", rn, measure.name()))
                            .arg(&format!("{}", device_id))
                            .execute(&redis_ctx.conn);

                        // lookup associated tank
                        let sensor_hash_key =
                            &format!("{}/sensors/{}/{}", rn, measure.name(), device_id).to_string();

                        let assoc_tank_num: Result<Vec<Option<u64>>, _> = redis_ctx.conn.hget(
                            sensor_hash_key,
                            vec!["tank", &format!("{}_update_count", measure.name())],
                        );

                        let ext_id_str: &str = &ext_id.external_id;
                        let _ = assoc_tank_num.iter().for_each(move |v| {
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
                                    .hget(&tank_key, &format!("{}_update_count", measure.name()));

                                let update: Result<String, _> = {
                                    let mut data: Vec<(
                                        &str,
                                        String,
                                    )> = vec![];
                                    /*
                                    ("temp_f", measure.temp_f.to_string()),
                                        ("temp_c", measure.temp_c.to_string())
                                    */
                                    data.push(unimplemented!());
                                    data.push((
                                        &format!("{}_update_count", measure.name()),
                                        tank_measure_count
                                            .unwrap_or(None)
                                            .map(|u| u + 1)
                                            .unwrap_or(1)
                                            .to_string(),
                                    ));
                                    data.push((
                                        &format!("{}_update_time", measure.name()),
                                        epoch_secs().to_string(),
                                    ));
                                    redis_ctx.conn.hset_multiple(&tank_key, &data[..])
                                };

                                if let Err(e) = update {
                                    println!("update fails for {}: {:?}", tank_key, e);
                                }
                            } else {
                                // We know that there's no associated "tank"
                                // field for this key.  Let's make sure the record
                                // for this sensor exists -- we'll need a human
                                // to come in and link this device to a specific tank
                                // using redis-cli!

                                redis_ctx.conn.exists(sensor_hash_key).iter().for_each(
                                    |e: &bool| {
                                        if !e {
                                            // new sensor, make note of when it is created
                                            let _: Result<
                                            Vec<bool>,
                                            _,
                                        > = redis_ctx.conn.hset_multiple(
                                            sensor_hash_key,
                                            &vec![
                                                ("create_time", format!("{}", epoch_secs())),
                                                ("ext_device_id", ext_id_str.to_string()),
                                            ][..],
                                        );
                                        }
                                    },
                                );
                            };

                            // record a hit on the updates that the sensor has seen
                            // and also record the most recent measurement on the record
                            // for this individual sensor
                            let update_sensor: Result<String, _> = {
                                let upd_c = &format!("{}_update_count", measure.name());
                                let mut data: Vec<(&str, String)> = vec![(
                                    upd_c,
                                    maybe_sensor_upd_count
                                        .map(|u| u + 1)
                                        .unwrap_or(1)
                                        .to_string(),
                                )];
                                data.push(unimplemented!());
                                /*
                                 ("temp_f", measure.temp_f.to_string()),
                                ("temp_c", measure.temp_c.to_string()),
                                */
                                data.push((
                                    &format!("{}_update_time", measure.name()),
                                    epoch_secs().to_string(),
                                ));

                                redis_ctx.conn.hset_multiple(sensor_hash_key, &data[..])
                            };
                            if let Err(e) = update_sensor {
                                println!(
                                    "couldn't update sensor record {}: {:?}",
                                    sensor_hash_key, e
                                );
                            }
                        });

                        println!("");
                    });
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
                    let _ = try_mqtt_reconnect(&mqtt_cli);
                }
            }
        }
    }
}

fn epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn try_mqtt_reconnect(cli: &paho_mqtt::Client) -> bool {
    println!("MQTT connection lost...");
    for i in 0..12 {
        println!("Retrying MQTT connection ({})", i);
        std::thread::sleep(std::time::Duration::from_millis(5000));
        if cli.reconnect().is_ok() {
            println!("MQTT successfully reconnected");
            return true;
        }
    }
    println!("Unable to reconnect MQTT after several attempts.");
    false
}
