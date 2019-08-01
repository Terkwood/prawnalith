#![feature(slice_concat_ext)]
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;
extern crate redis;

mod model;

use std::slice::SliceConcatExt;
use std::time;

use redis::Commands;
use rumqtt::{MqttClient, MqttOptions, QoS};

use uuid::Uuid;

use model::SensorReadings;

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
    warning: Option<String>,
    seconds_until_stale: Option<u32>,
}

fn generate_mq_client_id() -> String {
    format!("led_status/{}", Uuid::new_v4())
}

const AREAS: &str = "areas";

fn get_num_areas(conn: &redis::Connection, namespace: &str) -> Result<i64, redis::RedisError> {
    conn.get(format!("{}/{}", namespace, AREAS.to_string()))
}

struct Temp {
    f: f64,
    c: f64,
}

struct Staleness {
    warning: String,
    deadline_seconds: u32,
}

impl Staleness {
    fn text(&self, maybe_time: Option<u64>) -> String {
        match maybe_time {
            Some(update_time) if self.is_stale(update_time) => self.warning.to_owned(),
            _ => "".to_owned(),
        }
    }

    fn is_stale(&self, epoch_time_utc: u64) -> bool {
        epoch_secs() - epoch_time_utc > self.deadline_seconds as u64
    }
}

fn epoch_secs() -> u64 {
    time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn f_to_c(temp_f: f64) -> f64 {
    (temp_f - 32.0) * 5.0 / 9.0
}

fn c_to_f(temp_c: f64) -> f64 {
    temp_c * 1.8 + 32.0
}

const NAN: f64 = -255.0;

fn get_area_data(
    conn: &redis::Connection,
    area: i64,
    namespace: &str,
) -> Result<Option<SensorReadings>, redis::RedisError> {
    let numbers: Vec<Option<f64>> = conn.hget(
        format!("{}/areas/{}", namespace, area),
        vec![
            "humidity",
            "temp_f",
            "temp_c",
            "heat_index_f",
            "heat_index_c",
            "ph",
        ],
    )?;

    // A redis string
    let update_time_vec: Option<String> = conn.hget(
        format!("{}/areas/{}", namespace, area),
        vec!["sensors_update_time"],
    )?;

    let (humidity, init_temp_f, init_temp_c, heat_index_f, heat_index_c, ph) = (
        numbers.get(0),
        numbers.get(1),
        numbers.get(2),
        numbers.get(3),
        numbers.get(4),
        numbers.get(5),
    );

    let update_time = update_time_vec.map(|s| s.parse::<u64>().unwrap_or(0));

    let temp = safe_temp(init_temp_f, init_temp_c).map(|t| (t.f, t.c));

    Ok(Some(SensorReadings {
        humidity: unnest_ref(humidity),
        temp_f: temp.map(|t| t.0),
        temp_c: temp.map(|t| t.1),
        heat_index_f: unnest_ref(heat_index_f),
        heat_index_c: unnest_ref(heat_index_c),
        ph: unnest_ref(ph),
        update_time,
    }))
}
fn safe_temp(temp_f: Option<&Option<f64>>, temp_c: Option<&Option<f64>>) -> Option<Temp> {
    match (temp_f, temp_c) {
        (Some(&Some(f)), Some(&Some(c))) => Some(Temp { f, c }),
        (_, Some(&Some(c))) => Some(Temp { f: c_to_f(c), c }),
        (Some(&Some(f)), _) => Some(Temp { f, c: f_to_c(f) }),
        _ => None,
    }
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
    staleness: &Staleness,
) -> Result<String, redis::RedisError> {
    let num_areas = get_num_areas(&conn, namespace)?;

    let area_statuses: Result<Vec<String>, redis::RedisError> = (1..num_areas + 1)
        .map(move |area| {
            get_area_data(&conn, area, namespace).map(move |maybe_sensor_readings| {
                if let Some(sensor_readings) = maybe_sensor_readings {
                    let area_string = format!("A{}", area);

                    let stale = || staleness.text(sensor_readings.update_time);

                    let ph_string: String = sensor_readings
                        .ph
                        .map(move |ph| format!(" pH {}{}", ph, stale()))
                        .unwrap_or("".to_string());

                    let humidity_string: String = sensor_readings
                        .humidity
                        .map(move |h| format!(" {}%H{}", h, stale()))
                        .unwrap_or("".to_string());

                    // hope that both temps are present 🙈
                    let temp = match (sensor_readings.temp_c, sensor_readings.temp_f) {
                        (None, None) => None,
                        _ => Some(Temp {
                            f: sensor_readings.temp_f.unwrap_or(NAN),
                            c: sensor_readings.temp_c.unwrap_or(NAN),
                        }),
                    };

                    let heat_index =
                        match (sensor_readings.heat_index_c, sensor_readings.heat_index_f) {
                            (None, None) => None,
                            _ => Some(Temp {
                                f: sensor_readings.heat_index_f.unwrap_or(NAN),
                                c: sensor_readings.heat_index_c.unwrap_or(NAN),
                            }),
                        };

                    let temp_letter = temp_unit.to_ascii_uppercase();
                    let heat_index_string = heat_index
                        .map(move |hi| match temp_unit {
                            'c' | 'C' => hi.c,
                            _ => hi.f,
                        })
                        .map(|hi| format!(" {}h{}{}", hi, temp_letter, stale()))
                        .unwrap_or("".to_string());

                    let temp_string = temp
                        .map(move |t| match temp_unit {
                            'c' | 'C' => t.c,
                            _ => t.f,
                        })
                        .map(|t| format!(" {}°{}{}", t, temp_letter, stale()))
                        .unwrap_or("".to_string());

                    area_string + &ph_string + &humidity_string + &heat_index_string + &temp_string
                } else {
                    return "".to_string(); // nothing to format
                }
            })
        })
        .collect();

    area_statuses
        .map(|ss| ss.join(" "))
        .map(|msg| right_align(&msg)) // lay out the message nicely
}

fn right_align(message: &str) -> String {
    let l = message.to_string().len();
    if l <= 16 {
        format!("{: >16}", message)
    } else if l <= 32 {
        format!("{: >32}", message)
    } else if l <= 48 {
        format!("{: >48}", message)
    } else if l <= 64 {
        format!("{: >64}", message)
    } else if l <= 96 {
        format!("{: >96}", message)
    } else if l <= 128 {
        format!("{: >128}", message)
    } else {
        format!("{: >256}", message)
    }
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

    let staleness = {
        let warning = &config.warning.unwrap_or("[!]".to_owned());
        let deadline_seconds = &config.seconds_until_stale.unwrap_or(30);
        Staleness {
            warning: warning.to_string(),
            deadline_seconds: *deadline_seconds,
        }
    };

    loop {
        let status = generate_status(
            &redis_conn,
            &config.temp_unit.unwrap_or('F'),
            &config.redis_namespace.clone().unwrap_or("".to_owned()),
            &staleness,
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
