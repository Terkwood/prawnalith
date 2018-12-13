#![feature(slice_concat_ext)]
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;
extern crate redis;

use std::slice::SliceConcatExt;

use redis::Commands;
use rumqtt::{MqttClient, MqttOptions, QoS};

use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    redis_auth: Option<String>,
    redis_host: Option<String>,
    redis_port: Option<u16>,
    redis_namespace: Option<String>,
    mqtt_host: Option<String>,
    mqtt_port: Option<u16>,
    mqtt_topic: String,
    temp_unit: Option<char>,
    wait_secs: Option<u64>,
}

fn generate_mq_client_id() -> String {
    format!("led_status/{}", Uuid::new_v4())
}

fn get_num_tanks(conn: &redis::Connection, namespace: &str) -> Result<i64, redis::RedisError> {
    conn.get(format!("{}/tanks", namespace))
}

struct Temp {
    f: f64,
    c: f64,
    update_time: Option<u64>,
}

struct PH {
    val: f64,
    update_time: Option<u64>,
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
    namespace: &str,
) -> Result<(Option<Temp>, Option<PH>), redis::RedisError> {
    let numbers: Vec<Option<f64>> = conn.hget(
        format!("{}/tanks/{}", namespace, tank),
        vec!["temp_f", "temp_c", "ph"],
    )?;
    let update_times: Vec<Option<u64>> = conn.hget(
        format!("{}/tanks/{}", namespace, tank),
        vec!["temp_update_time", "ph_update_time"],
    )?;
    let (temp_f, temp_c) = (numbers.get(0), numbers.get(1));
    let (temp_update_time, ph_update_time) = (
        unnest_ref(update_times.get(0)),
        unnest_ref(update_times.get(1)),
    );
    let temp = match (temp_f, temp_c) {
        (Some(&Some(f)), Some(&Some(c))) => Some(Temp {
            f,
            c,
            update_time: temp_update_time,
        }),
        (_, Some(&Some(c))) => Some(Temp {
            f: c_to_f(c),
            c,
            update_time: temp_update_time,
        }),
        (Some(&Some(f)), _) => Some(Temp {
            f,
            c: f_to_c(f),
            update_time: temp_update_time,
        }),
        _ => None,
    };
    let ph = unnest_ref(numbers.get(2)).map(|val| PH {
        val,
        update_time: ph_update_time,
    });

    Ok((temp, ph))
}

fn unnest_ref<A>(a: Option<&Option<A>>) -> Option<A>
where
    A: Copy,
{
    match a {
        Some(&Some(thing)) => Some(thing),
        Some(&None) => None,
        None => None,
    }
}

fn generate_status(
    conn: &redis::Connection,
    temp_unit: &char,
    namespace: &str,
) -> Result<String, redis::RedisError> {
    let num_tanks = get_num_tanks(&conn, namespace)?;

    let status_results: Result<Vec<String>, redis::RedisError> = (1..num_tanks + 1)
        .map(move |tank| {
            get_temp_ph(&conn, tank, namespace).map(move |(maybe_temp, maybe_ph)| {
                if let (&None, &None) = (&maybe_temp, &maybe_ph) {
                    return "".to_string(); // nothing to format
                }

                let tank_string = format!("#{}:", tank);
                let temp_string = maybe_temp
                    .map(move |t| match temp_unit {
                        'c' | 'C' => t.c,
                        _ => t.f,
                    })
                    .map(|t| format!(" {}°{}", t, temp_unit.to_ascii_uppercase()))
                    .unwrap_or("".to_string());
                let ph_string: String = maybe_ph
                    .map(move |level| format!(" pH {}", level.val))
                    .unwrap_or("".to_string());

                let message = tank_string + &temp_string + &ph_string;

                // finally, right-align the message so it lays out nicely on the LEDs
                let l = message.to_string().len();
                if l <= 16 {
                    format!("{: >16}", message)
                } else if l <= 32 {
                    format!("{: >32}", message)
                } else if l <= 64 {
                    format!("{: >64}", message)
                } else {
                    format!("{: >128}", message)
                }
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
    let redis_conn = redis_client.get_connection().unwrap();

    let mut mq_cli = {
        // Specify client connection options
        let opts: MqttOptions = MqttOptions::new()
            .set_keep_alive(5)
            .set_reconnect(3)
            .set_client_id(generate_mq_client_id())
            .set_broker(
                &format!(
                    "{}:{}",
                    &config.mqtt_host.unwrap_or("127.0.0.1".to_string()),
                    &config.mqtt_port.unwrap_or(1883)
                )[..],
            );
        MqttClient::start(opts, None).expect("MQTT client couldn't start")
    };

    let wait_secs = config.wait_secs.unwrap_or(10);

    loop {
        let status = generate_status(
            &redis_conn,
            &config.temp_unit.unwrap_or('F'),
            &config.redis_namespace.clone().unwrap_or("".to_string()),
        );
        mq_cli
            .publish(
                &config.mqtt_topic,
                QoS::Level0,
                status.unwrap().clone().into_bytes(),
            )
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(wait_secs));
    }
}
