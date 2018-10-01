#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;

#[derive(Deserialize, Debug)]
struct Config {
    redis_auth: Option<String>,
    redis_host: Option<String>,
    redis_port: Option<u16>,
    mqtt_host: Option<String>,
    mqtt_port: Option<u16>,
    mqtt_topic: String,
}

fn main() {
    dotenv::dotenv().expect("Unable to read .env file");

    match envy::from_env::<Config>() {
        Ok(config) => println!("{:?}", config),
        Err(e) => println!("Unable to parse config ({})", e),
    };
}
