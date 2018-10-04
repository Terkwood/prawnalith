#![feature(slice_concat_ext)]
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;
extern crate redis;
extern crate rumqtt;
extern crate uuid;

use rumqtt::MqttCallback;

use redis::Commands;
use uuid::Uuid;

mod config;
mod prawnqtt;
mod predis;

/// parses a hexadecimal string  (e.g. "28654597090000e4") into
/// a UUID v5.  the hex string is used to make a small UUID,
/// which serves as the namespace for the resulting V5 UUID.
/// this makes translation relatively easy between external
/// id and internal ID.
fn generate_sensor_id(
    hex_string: &str,
    mapping_name: &str,
) -> Result<Uuid, uuid::parser::ParseError> {
    let uuid_namespace = Uuid::parse_str(&format!("0000000000000000{}", hex_string)[..])?;
    Ok(Uuid::new_v5(&uuid_namespace, mapping_name.as_bytes()))
}

struct RedisOpts {
    conn: redis::Connection,
    namespace: String,
}

/*
{
        let ext_sensor_id = "28654597090000e4";
        println!("external sensor id             : {}", ext_sensor_id);
        
        println!(
            "external sensor id (tiny UUID) : {}",
            Uuid::parse_str(&format!("0000000000000000{}", "28654597090000e4")[..]).unwrap()
        );
        println!(
            "mapping name                   : {}",
            sensor_id_mapping_name
        );
        println!(
            "internal sensor ID             : {}",
            generate_sensor_id(ext_sensor_id, sensor_id_mapping_name).unwrap()
        );
    }
    */

/// This is the "name" field that will be used to form a V5 UUID
fn get_external_device_namespace(opts: &RedisOpts) -> Result<Uuid, redis::RedisError> {
    let key = format!("{}/external_device_namespace", opts.namespace);
    let r: Option<String> = opts.conn.get(&key)?;

    match r {
        None => {
            let it = Uuid::new_v4();
            opts.conn.set(key, it.to_string())?;
            Ok(it)
        }
        Some(s) => {
            Ok(Uuid::parse_str(&s[..]).unwrap()) // fine.  just panic then.
        }
    }
}

/// `external_device_id` is usually reported as a
/// e.g. "28654597090000e4"
fn compute_internal_id(
    external_device_id: &str,
    external_device_namespace: Uuid,
) -> Result<Uuid, uuid::parser::ParseError> {
    Ok(Uuid::new_v5(
        &external_device_namespace,
        external_device_id.as_bytes(),
    ))
}

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = match envy::from_env::<config::Config>() {
        Ok(config) => config,
        Err(e) => panic!("Unable to parse config ({})", e),
    };

    // DEFAULT CONFIGURATIONS LIVE HERE!
    let mq_host = &config.mqtt_host.unwrap_or("127.0.0.1".to_string());
    let mq_port = &config.mqtt_port.unwrap_or(1883);
    // mqtt spec states that this is measured in secs
    // see http://www.steves-internet-guide.com/mqtt-keep-alive-by-example/
    let mq_keep_alive = &config.mqtt_keep_alive.unwrap_or(10);
    let mq_topic = &config.mqtt_topic;

    // Set up redis client
    let redis_client = {
        let redis_host = &config.redis_host.unwrap_or("127.0.0.1".to_string());
        let redis_port: u16 = config.redis_port.unwrap_or(6379);
        let redis_auth: Option<String> = config.redis_auth;

        let rci = redis::ConnectionInfo {
            addr: Box::new(redis::ConnectionAddr::Tcp(
                redis_host.to_string(),
                redis_port,
            )),
            db: 0,
            passwd: redis_auth,
        };
        redis::Client::open(rci).unwrap()
    };
    let redis_opts = RedisOpts {
        conn: redis_client.get_connection().unwrap(),
        namespace: config.redis_namespace.unwrap_or("".to_string()),
    };

    let external_device_namespace = get_external_device_namespace(&redis_opts).unwrap();
    println!("external device namespace is {}", external_device_namespace);

    let mut mq_message_callback = MqttCallback::new().on_message(|msg| {
        println!("Received payload: {:?}", msg);
    });
    let mut mq_req_handler =
        prawnqtt::mq_request_handler(mq_message_callback, mq_host, *mq_port, *mq_keep_alive);
}
