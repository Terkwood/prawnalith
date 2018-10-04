use super::model::TempMessage;
use rumqtt::{MqttCallback, MqttClient, MqttOptions};
use uuid::Uuid;

pub fn mq_client(mq_host: &str, mq_port: u16, mq_keep_alive: u16) -> rumqtt::MqttClient {
    let callback = |msg: rumqtt::Message| {
        println!("Received payload:\n\t{:?}", msg);
        let deser: Result<TempMessage, _> =
            serde_json::from_str(std::str::from_utf8(&*msg.payload).unwrap());
        match deser {
            Err(_) => println!("\t[!] couldn't deserialize [!]"),
            Ok(temp) => {
                println!("\t{:?}", temp);
                unimplemented!()
            }
        }
    };
    let mq_message_callback = MqttCallback::new().on_message(callback);

    // Specify client connection options
    let opts: MqttOptions = MqttOptions::new()
        .set_keep_alive(mq_keep_alive)
        .set_reconnect(3)
        .set_client_id(generate_mq_client_id())
        .set_broker(&format!("{}:{}", mq_host, mq_port)[..]);
    MqttClient::start(opts, Some(mq_message_callback)).expect("MQTT client couldn't start")
}

fn generate_mq_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}
