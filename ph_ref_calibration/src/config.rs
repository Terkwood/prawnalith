#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    redis_auth: Option<String>,
    redis_host: Option<String>,
    redis_port: Option<u16>,
    redis_namespace: Option<String>,
}
