use super::model;
use crossbeam_channel as channel;
use rumqtt::MqttCallback;

fn deser_message(msg: &rumqtt::Message) -> Option<model::TempMessage> {
    serde_json::from_str(std::str::from_utf8(&*msg.payload).unwrap())
        .map(|r| Some(r))
        .unwrap_or(None)
}

pub fn create_mqtt_callback(update_s: channel::Sender<model::TempMessage>) -> MqttCallback {
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

    MqttCallback::new().on_message(deserialize_and_forward)
}
