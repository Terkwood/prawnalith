#![feature(slice_concat_ext)]
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;
extern crate redis;

use redis::Commands;
use std::slice::SliceConcatExt;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    redis_auth: Option<String>,
    redis_host: Option<String>,
    redis_port: Option<u16>,
    mqtt_host: Option<String>,
    mqtt_port: Option<u16>,
    mqtt_topic: String,
    temp_unit: Option<TempUnit>,
}

#[derive(Deserialize, Debug, Clone)]
enum TempUnit {
    F,
    C,
}

impl Default for TempUnit {
    fn default() -> TempUnit {
        TempUnit::F
    }
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
            temp_unit: None,
        }
    }
}

fn redis_connection_string(config: &Config) -> String {
    let auth_string = match &config.redis_auth {
        Some(a) => format!(":{}@", a),
        None => "".to_string(),
    };

    let host_portion: &String = &config
        .redis_host
        .clone()
        .unwrap_or(Config::default().redis_host.unwrap());
    let port_portion: u16 = config
        .redis_port
        .unwrap_or(Config::default().redis_port.unwrap());

    format!("redis://{}{}:{}", auth_string, host_portion, port_portion)
}

fn get_num_tanks(conn: &redis::Connection) -> Result<i64, redis::RedisError> {
    conn.get("prawnalith/tanks")
}

struct Temp {
    f: f64,
    c: f64,
}

fn f_to_c(temp_f: f64) -> f64 {
    (temp_f - 32.0) * 5.0 / 9.0
}

fn c_to_f(temp_c: f64) -> f64 {
    temp_c * 1.8 + 32.0
}

fn get_temp_ph(
    conn: &redis::Connection,
    tank: i64,
) -> Result<(Option<Temp>, Option<f64>), redis::RedisError> {
    let numbers: Vec<Option<f64>> = conn.hget(
        format!("prawnalith/tanks/{}", tank),
        vec!["temp_f", "temp_c", "ph"],
    )?;
    let (temp_f, temp_c) = (numbers.get(0), numbers.get(1));
    let temp = match (temp_f, temp_c) {
        (Some(&Some(f)), Some(&Some(c))) => Some(Temp { f, c }),
        (_, Some(&Some(c))) => Some(Temp { f: c_to_f(c), c }),
        (Some(&Some(f)), _) => Some(Temp { f, c: f_to_c(f) }),
        _ => None,
    };
    let ph = match numbers.get(2) {
        Some(&Some(level)) => Some(level),
        Some(&None) => None,
        None => None,
    };

    Ok((temp, ph))
}

fn generate_status(
    conn: &redis::Connection,
    temp_unit: &TempUnit,
) -> Result<String, redis::RedisError> {
    let num_tanks = get_num_tanks(&conn)?;

    let status_results: Result<Vec<String>, redis::RedisError> = (1..num_tanks + 1)
        .map(move |tank| {
            get_temp_ph(&conn, tank).map(move |(maybe_temp, maybe_ph)| {
                if let (&None, &None) = (&maybe_temp, &maybe_ph) {
                    return "".to_string(); // nothing to format
                }

                let temp_string = maybe_temp
                    .map(move |t| match temp_unit {
                        TempUnit::F => t.f,
                        TempUnit::C => t.c,
                    })
                    .map(|t| format!(" {}", t))
                    .unwrap_or("".to_string());
                let ph_string: String = maybe_ph
                    .map(move |level| format!(" {}", level))
                    .unwrap_or("".to_string());

                format!("#{}:{}{}", tank, temp_string, ph_string)
            })
        })
        .collect();

    status_results.map(|ss| ss.join(" "))
}

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(e) => panic!("Unable to parse config ({})", e),
    };

    let redis_client = redis::Client::open(&redis_connection_string(&config)[..]).unwrap();
    let redis_conn = redis_client.get_connection().unwrap();

    let status = generate_status(
        &redis_conn,
        &config.temp_unit.unwrap_or(TempUnit::default()),
    );
    println!("{}", status.unwrap());
}
