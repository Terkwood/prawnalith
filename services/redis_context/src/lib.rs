extern crate redis;
extern crate uuid;

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
    pub fn get_external_device_namespace(
        &self,
        device_type: String,
    ) -> Result<Uuid, redis::RedisError> {
        let key = format!("{}/external_device_namespace", self.namespace);
        let r: Option<String> = self.conn.hget(&key, device_type)?;

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


pub enum ExternalDevice {
    Temp,
    PH,
    Unknown
}


impl From<String> for ExternalDevice {
    fn from(device_type: String) -> Self {
        match device_type.to_lowercase().trim() {
            "temp" => ExternalDevice::Temp,
            "ph" => ExternalDevice::PH,
            _ => ExternalDevice::Unknown
        }
    }
}
