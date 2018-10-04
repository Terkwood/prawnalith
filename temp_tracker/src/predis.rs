use redis;

use super::model;
use crossbeam_channel as channel;
use redis::Commands;
use uuid::Uuid;

pub struct RedisContext {
    pub conn: redis::Connection,
    pub namespace: String,
}
impl RedisContext {
    pub fn new(host: String, port: u16, auth: Option<String>, namespace: String) -> RedisContext {
        RedisContext {
            conn: {
                let rci = redis::ConnectionInfo {
                    addr: Box::new(redis::ConnectionAddr::Tcp(host.to_string(), port)),
                    db: 0,
                    passwd: auth,
                };
                redis::Client::open(rci).unwrap().get_connection().unwrap()
            },
            namespace: namespace,
        }
    }

    /// This is the "name" field that will be used to form a V5 UUID
    pub fn get_external_device_namespace(&self) -> Result<Uuid, redis::RedisError> {
        let key = format!("{}/external_device_namespace", self.namespace);
        let r: Option<String> = self.conn.get(&key)?;

        match r {
            None => {
                let it = Uuid::new_v4();
                self.conn.set(key, it.to_string())?;
                Ok(it)
            }
            Some(s) => {
                Ok(Uuid::parse_str(&s[..]).unwrap()) // fine.  just panic then.
            }
        }
    }
}

/// We declare this crossbeam_channel update receiver
/// so that we avoid a hellish realm of static lifetimes,
/// dropped borrows, and wielded sorrows.
pub fn receive_updates(update_r: channel::Receiver<model::TempMessage>, redis_ctx: &RedisContext) {
    loop {
        match update_r.recv() {
            Some(temp) => {
                println!("\tReceived redis temp update: {:?}", temp);
                let device_id = temp
                    .id(&redis_ctx.get_external_device_namespace().unwrap())
                    .unwrap();
                println!("\tInternal ID for device: {}", device_id);
                let rn = &redis_ctx.namespace;

                // add this sensor to member set if it's never been seen before
                //unimplemented!();

                // lookup associated tank
                let assoc_tank_num: Result<Option<u16>, _> = redis_ctx
                    .conn
                    .hget(format!("{}/temp_sensors/{}", rn, device_id), "tank");

                let _ = assoc_tank_num
                    .iter()
                    .flatten()
                    .map(|t| println!("tank # {}", t));
                println!("");
            }
            _ => {}
        }
    }
}
