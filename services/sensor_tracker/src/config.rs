use redis_context::RedisContext;

#[derive(Deserialize, Debug, Clone)]
pub struct TrackerConfig {
    pub redis_auth: Option<String>,
    pub redis_host: Option<String>,
    pub redis_port: Option<u16>,
    pub redis_namespace: Option<String>,
    pub redis_delta_event_topic: Option<String>,
    pub mqtt_host: Option<String>,
    pub mqtt_port: Option<u16>,
    pub mqtt_topic: String,
    pub mqtt_keep_alive: Option<u16>,
    pub mqtt_qos: Option<u16>,
}

impl TrackerConfig {
    pub fn new() -> TrackerConfig {
        match envy::from_env::<TrackerConfig>() {
            Ok(config) => config,
            Err(e) => panic!("Unable to parse config ({})", e),
        }
    }

    pub fn to_redis_context(&self) -> RedisContext {
        RedisContext::new(
            (self.redis_host.clone().unwrap_or("127.0.0.1".to_string())).to_string(),
            self.redis_port.unwrap_or(6379),
            self.redis_auth.clone(),
            self.redis_namespace.clone().unwrap_or("".to_string()),
        )
    }
}
