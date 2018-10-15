#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub redis_auth: Option<String>,
    pub redis_host: Option<String>,
    pub redis_port: Option<u16>,
    pub redis_namespace: Option<String>,
}
