use crossbeam_channel::Receiver;
use redis_context::RedisContext;
use rumqtt::Notification;

use crate::model::SensorMessage;
use crate::predis;

pub fn receive_updates(
    update_r: Receiver<Notification>,
    redis_ctx: &RedisContext,
    delta_event_topic: &str,
) {
    loop {
        match update_r.try_recv() {
            // TODO use select! macro
            Ok(Notification::Publish(p)) => {
                let payload = p.payload;
                if let Some(sensor_message) = deser_message(&payload) {
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
                } else {
                    println!("couldnt deserialize message payload: {:?}", payload)
                }
            }
            Ok(n) => println!("IGNORE  {:?}", n),
            Err(crossbeam_channel::TryRecvError::Empty) => (),
            Err(e) => println!("ERROR    {:?}", e),
        }

        std::thread::sleep(std::time::Duration::from_millis(100))
    }
}

fn deser_message(payload: &[u8]) -> Option<SensorMessage> {
    let r = std::str::from_utf8(&payload);
    r.ok()
        .and_then(|s| serde_json::from_str(s).map(|r| Some(r)).unwrap_or(None))
}
