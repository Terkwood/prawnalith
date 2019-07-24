use super::prawnqtt;
use super::predis;
use redis_context::RedisContext;
use crossbeam_channel::Receiver;

use crate::prawnqtt::{Client, Message};

pub fn receive_updates(
    update_r: Receiver<Option<Message>>,
    redis_ctx: &RedisContext,
    mqtt_cli: Client,
    delta_event_topic: &str,
) {
    loop {
        match update_r.try_recv() {
            Ok(Some(paho)) => {
                if let Some(sensor_message) = prawnqtt::deser_message(paho) {
                    let ext_device_id: &str = &sensor_message.device_id;

                    sensor_message.measurements().iter().for_each(|measure| {
                        if let Ok(delta_events) = predis::update(redis_ctx, &measure, ext_device_id)
                        {
                            // emit all changed keys & hash field names to redis
                            // on the appropriate redis pub/sub topic.
                            // these will be processed later by the gcloud_push utility
                            predis::publish_updates(redis_ctx, delta_event_topic, delta_events)
                        }
                    });
                }
            }

            Err(_) if unimplemented!()  => {
                            // TODO
            /* check for 
               !mqtt_cli.is_connected()
               */
                let _ = try_mqtt_reconnect(&mqtt_cli);
            }
            _ => (),
        }

        std::thread::sleep(std::time::Duration::from_millis(100))
    }
}

// TODO trim
/*
fn try_mqtt_reconnect(cli: &Client) -> bool {
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
*/
