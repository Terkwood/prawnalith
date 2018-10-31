use tracker_support::epoch_secs;

use redis;
use redis::Commands;

use super::prawnqtt;
use redis_context::RedisContext;

pub fn receive_updates(
    update_r: std::sync::mpsc::Receiver<Option<paho_mqtt::message::Message>>,
    redis_ctx: &RedisContext,
) {
    loop {
        match update_r.try_recv() {
            Ok(Some(paho)) => {
                if let Some(temp) = prawnqtt::deser_message(paho) {
                    println!("\tReceived redis temp update: {:?}", temp);
                    let device_id: String = format!(
                        "{}",
                        temp.id(&redis_ctx
                            .get_external_device_namespace("temp".to_string())
                            .unwrap())
                            .unwrap()
                    );
                    println!("\tDevice ID (internal): {}", device_id);
                    let rn = &redis_ctx.namespace;

                    // add to the member set if it doesn't already exist
                    let _ = redis::cmd("SADD")
                        .arg(format!("{}/sensors/temp", rn))
                        .arg(&device_id)
                        .execute(&redis_ctx.conn);

                    // lookup associated tank
                    let temp_sensor_hash_key =
                        &format!("{}/sensors/temp/{}", rn, device_id).to_string();

                    let assoc_tank_num: Result<Vec<Option<u64>>, _> = redis_ctx
                        .conn
                        .hget(temp_sensor_hash_key, vec!["tank", "temp_update_count"]);

                    let _ = assoc_tank_num.iter().for_each(|v| {
                        let maybe_tank_num = v.get(0).unwrap_or(&None);
                        let maybe_temp_sensor_uc: &Option<_> = v.get(1).unwrap_or(&None);
                        if let Some(tank_num) = maybe_tank_num {
                            // We found the tank associated with this
                            // sensor ID, so we should update that tank's
                            // current temp reading.
                            let tank_key = format!("{}/tanks/{}", rn, tank_num);

                            let tank_temp_count: Result<Option<u32>, _> =
                                redis_ctx.conn.hget(&tank_key, "temp_update_count");

                            let update: Result<String, _> = redis_ctx.conn.hset_multiple(
                                &tank_key,
                                &vec![
                                    ("temp_f", temp.temp_f.to_string()),
                                    ("temp_c", temp.temp_c.to_string()),
                                    ("temp_update_time", epoch_secs().to_string()),
                                    (
                                        "temp_update_count",
                                        tank_temp_count
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

                            redis_ctx.conn.exists(temp_sensor_hash_key).iter().for_each(
                                |e: &bool| {
                                    if !e {
                                        // new temp sensor, make note of when it is created
                                        let _: Result<
                                            Vec<bool>,
                                            _,
                                        > = redis_ctx.conn.hset_multiple(
                                            temp_sensor_hash_key,
                                            &vec![
                                                ("create_time", format!("{}", epoch_secs())),
                                                ("ext_device_id", temp.device_id.to_string()),
                                            ][..],
                                        );
                                    }
                                },
                            );
                        };

                        // record a hit on the temp updates that the sensor has seen
                        // and also record the most recent temp on the sensor itself
                        let update_sensor: Result<String, _> = redis_ctx.conn.hset_multiple(
                            temp_sensor_hash_key,
                            &vec![
                                (
                                    "temp_update_count",
                                    maybe_temp_sensor_uc.map(|u| u + 1).unwrap_or(1).to_string(),
                                ),
                                ("temp_f", temp.temp_f.to_string()),
                                ("temp_c", temp.temp_c.to_string()),
                                ("temp_update_time", epoch_secs().to_string()),
                            ][..],
                        );
                        if let Err(e) = update_sensor {
                            println!(
                                "couldn't update sensor record {}: {:?}",
                                temp_sensor_hash_key, e
                            );
                        }
                    });
                    println!("");
                }
            }
            other => println!("No recv match: {:?}", other),
        }
    }
}
