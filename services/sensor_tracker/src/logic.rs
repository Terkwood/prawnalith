use super::prawnqtt;
use super::predis;
use redis_context::RedisContext;
use std::sync::mpsc::Receiver;

pub fn receive_updates(
    update_r: Receiver<Option<paho_mqtt::message::Message>>,
    redis_ctx: &RedisContext,
    mqtt_cli: paho_mqtt::Client,
) {
    loop {
        match update_r.try_recv() {
            Ok(Some(paho)) => {
                if let Some(sensor_message) = prawnqtt::deser_message(paho) {
                    let ext_device_id: &str = &sensor_message.device_id;

                    sensor_message.measurements().iter().for_each(|measure| {
                        let delta_events = predis::update(redis_ctx, &measure, ext_device_id);
                    });
                }
            }
            Err(_) if !mqtt_cli.is_connected() => {
                let _ = try_mqtt_reconnect(&mqtt_cli);
            }
            _ => (),
        }

        std::thread::sleep(std::time::Duration::from_millis(100))
    }
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
