use super::model;
use crossbeam_channel as channel;
use rumqtt::{MqttCallback, MqttClient, MqttOptions, QoS};
use uuid::Uuid;

pub fn start_mqtt(
    update_s: channel::Sender<model::TempMessage>,
    mq_host: &str,
    mq_port: u16,
    mq_topic: &str,
    mq_keep_alive: u16,
) {
    let on_temp_update = move |msg: rumqtt::Message| {
        println!("Received payload:\n\t{:?}", msg);
        let deser: Result<model::TempMessage, _> =
            serde_json::from_str(std::str::from_utf8(&*msg.payload).unwrap());
        match deser {
            Err(_) => println!("\t[!] couldn't deserialize [!]"),
            Ok(temp) => {
                println!("\t{:?}", temp);

                update_s.send(temp)
            }
        }
    };

    let mq_message_callback = MqttCallback::new().on_message(on_temp_update);

    // Specify client connection options
    let opts: MqttOptions = MqttOptions::new()
        .set_keep_alive(mq_keep_alive)
        .set_reconnect(3)
        .set_client_id(generate_mq_client_id())
        .set_broker(&format!("{}:{}", mq_host, mq_port)[..]);

    MqttClient::start(opts, Some(mq_message_callback))
        .expect("MQTT client couldn't start")
        .subscribe(vec![(mq_topic, QoS::Level0)])
        .unwrap()
}

fn generate_mq_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}
