/// Note that _most of the configuration is given
/// in Rocket.toml_.  [See documentation at rocket.rs](https://rocket.rs/v0.4/guide/state/#usage)
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub redis_namespace: Option<String>,
}
