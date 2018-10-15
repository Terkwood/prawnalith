use super::model;
use crossbeam_channel as channel;
use rumqtt::{MqttCallback, MqttClient, MqttOptions, QoS};
use uuid::Uuid;

fn deser_message(msg: &rumqtt::Message) -> Option<model::TempMessage> {
    serde_json::from_str(std::str::from_utf8(&*msg.payload).unwrap())
        .map(|r| Some(r))
        .unwrap_or(None)
}

pub fn start_mqtt(
    update_s: channel::Sender<model::TempMessage>,
    mq_host: &str,
    mq_port: u16,
    mq_topic: &str,
    mq_keep_alive: u16,
) {
    let deserialize_and_forward = move |msg: rumqtt::Message| {
        println!("Message on {:?}", msg.topic);
        let deser: Option<model::TempMessage> = deser_message(&msg);
        match deser {
            None => println!(
                "\t[!] couldn't deserialize payload [!]\n\t[!]\t{:?}\t[!]",
                msg
            ),
            Some(temp) =>
            // forward the message to someone who can handle it
            // without having to deal with sync restrictions
            // on our local redis connection, etc
            {
                update_s.send(temp)
            }
        }
    };

    let mq_message_callback = MqttCallback::new().on_message(deserialize_and_forward);

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
