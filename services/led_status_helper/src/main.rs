#![feature(slice_concat_ext)]
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;
extern crate redis;

use std::slice::SliceConcatExt;
use std::time;

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
    warning: Option<String>,
    seconds_until_stale: Option<u32>,
}

fn generate_mq_client_id() -> String {
    format!("led_status/{}", Uuid::new_v4())
}

fn get_num_containers(
    conn: &redis::Connection,
    namespace: &str,
    container: Container,
) -> Result<i64, redis::RedisError> {
    conn.get(format!("{}/{}", namespace, container.to_string()))
}

enum Container {
    Tanks,
    Areas,
}

impl Container {
    pub fn to_string(self) -> String {
        match self {
            Container::Tanks => "tanks".to_string(),
            Container::Areas => "areas".to_string(),
        }
    }
}

struct Temp {
    f: f64,
    c: f64,
    update_time: Option<u64>,
}

/// Digital humidity and temp, e.g. DHT11 sensor
struct DHT {
    humidity: f64,
    temp_f: f64,
    temp_c: f64,
    heat_index_f: f64,
    heat_index_c: f64,
    update_time: Option<u64>,
}

struct PH {
    val: f64,
    update_time: Option<u64>,
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
) -> Result<Option<DHT>, redis::RedisError> {
    let numbers: Vec<Option<f64>> = conn.hget(
        format!("{}/areas/{}", namespace, area),
        vec![
            "humidity",
            "temp_f",
            "temp_c",
            "heat_index_f",
            "heat_index_c",
        ],
    )?;

    // A redis string
    let update_time_vec: Option<String> = conn.hget(
        format!("{}/areas/{}", namespace, area),
        vec!["dht_update_time"],
    )?;

    let (humidity, init_temp_f, init_temp_c, heat_index_f, heat_index_c) = (
        numbers.get(0),
        numbers.get(1),
        numbers.get(2),
        numbers.get(3),
        numbers.get(4),
    );

    let update_time = update_time_vec.map(|s| s.parse::<u64>().unwrap_or(0));

    let temp = safe_temp(init_temp_f, init_temp_c, update_time);

    let (temp_f, temp_c) = temp.map(|t| (t.f, t.c)).unwrap_or((NAN, NAN));

    Ok(Some(DHT {
        humidity: unnest_ref(humidity).unwrap_or(NAN),
        temp_f,
        temp_c,
        heat_index_f: unnest_ref(heat_index_f).unwrap_or(NAN),
        heat_index_c: unnest_ref(heat_index_c).unwrap_or(NAN),
        update_time,
    }))
}

fn get_tank_data(
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
    let temp = safe_temp(temp_f, temp_c, temp_update_time);
    let ph = unnest_ref(numbers.get(2)).map(|val| PH {
        val,
        update_time: ph_update_time,
    });

    Ok((temp, ph))
}

fn safe_temp(
    temp_f: Option<&Option<f64>>,
    temp_c: Option<&Option<f64>>,
    temp_update_time: Option<u64>,
) -> Option<Temp> {
    match (temp_f, temp_c) {
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
    let num_tanks = get_num_containers(&conn, namespace, Container::Tanks)?;

    let tank_statuses: Result<Vec<String>, redis::RedisError> = (1..num_tanks + 1)
        .map(move |tank| {
            get_tank_data(&conn, tank, namespace).map(move |(maybe_temp, maybe_ph)| {
                if let (&None, &None) = (&maybe_temp, &maybe_ph) {
                    return "".to_string(); // nothing to format
                }

                let tank_string = format!("T{}:", tank);
                let temp_string = maybe_temp
                    .map(move |t| {
                        (
                            match temp_unit {
                                'c' | 'C' => t.c,
                                _ => t.f,
                            },
                            t.update_time,
                        )
                    })
                    .map(|(t, update_time)| {
                        format!(
                            " {}°{}{}",
                            t,
                            temp_unit.to_ascii_uppercase(),
                            staleness.text(update_time)
                        )
                    })
                    .unwrap_or("".to_string());
                let ph_string: String = maybe_ph
                    .map(move |ph| format!(" pH {}{}", ph.val, staleness.text(ph.update_time)))
                    .unwrap_or("".to_string());

                tank_string + &ph_string + &temp_string
            })
        })
        .collect();

    let tank_portion = tank_statuses.map(|ss| ss.join(" "));

    let num_areas = get_num_containers(&conn, namespace, Container::Areas)?;

    let area_statuses: Result<Vec<String>, redis::RedisError> = (1..num_areas + 1)
        .map(move |area| {
            get_area_data(&conn, area, namespace).map(move |maybe_dht| {
                if let &None = &maybe_dht {
                    return "".to_string(); // nothing to format
                }

                let area_string = format!("A{}: ", area);

                let data_string = maybe_dht
                    .map(move |dht| {
                        (
                            dht.humidity,
                            match temp_unit {
                                'c' | 'C' => dht.temp_c,
                                _ => dht.temp_f,
                            },
                            match temp_unit {
                                'c' | 'C' => dht.heat_index_c,
                                _ => dht.heat_index_f,
                            },
                            dht.update_time,
                        )
                    })
                    .map(|(humidity, temp, heat_index, update_time)| {
                        let stale = staleness.text(update_time);
                        let temp_letter = temp_unit.to_ascii_uppercase();
                        format!(
                            "{}%H{} {}°{}{} {}h{}{}",
                            humidity,
                            stale,
                            temp,
                            temp_letter,
                            stale,
                            heat_index,
                            temp_letter,
                            stale,
                        )
                    })
                    .unwrap_or("".to_string());

                area_string + &data_string
            })
        })
        .collect();

    let area_portion = area_statuses.map(|ss| ss.join(" "));

    tank_portion
        .and_then(|tp| area_portion.map(|ap| ap + " " + &tp)) // join areas and tanks
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
