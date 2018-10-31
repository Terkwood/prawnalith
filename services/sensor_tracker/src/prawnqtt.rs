use super::model;
use paho_mqtt;

pub fn deser_message(msg: paho_mqtt::Message) -> Option<model::TempMessage> {
    serde_json::from_str(std::str::from_utf8(&*msg.payload()).unwrap())
        .map(|r| Some(r))
        .unwrap_or(None)
}
