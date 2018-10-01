#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;
extern crate redis;

use redis::Commands;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    redis_auth: Option<String>,
    redis_host: Option<String>,
    redis_port: Option<u16>,
    mqtt_host: Option<String>,
    mqtt_port: Option<u16>,
    mqtt_topic: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            redis_auth: None,
            redis_host: Some("127.0.0.1".to_string()),
            redis_port: Some(6379),
            mqtt_host: Some("127.0.0.1".to_string()),
            mqtt_port: Some(1883),
            mqtt_topic: "led_message".to_string(),
        }
    }
}

fn redis_connection_string(config: Config) -> String {
    let auth_string = match config.redis_auth {
        Some(a) => format!(":{}@", a),
        None => "".to_string(),
    };

    format!(
        "redis://{}{}:{}",
        auth_string,
        config
            .redis_host
            .unwrap_or(Config::default().redis_host.unwrap()),
        config
            .redis_port
            .unwrap_or(Config::default().redis_port.unwrap())
    )
}

fn get_num_tanks(conn: &redis::Connection) -> Result<i64, redis::RedisError> {
    conn.get("prawnalith/tanks")
}

fn get_temp_ph(conn: &redis::Connection, tank: i64) -> Result<Vec<Option<f64>>, redis::RedisError> {
    conn.hget(format!("prawnalith/tanks/{}", tank), vec!["temp", "ph"])
}

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(e) => panic!("Unable to parse config ({})", e),
    };

    let redis_client = redis::Client::open(&redis_connection_string(config)[..]).unwrap();
    let redis_conn = redis_client.get_connection().unwrap();

    let num_tanks = get_num_tanks(&redis_conn).unwrap();
    (1..num_tanks + 1)
        .for_each(move |t| println!("Tank {}: {:?}", t, get_temp_ph(&redis_conn, t).unwrap()));
}
