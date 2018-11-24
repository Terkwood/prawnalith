use crate::redis_conn::RedisPoolContext;
use regex::Regex;

/// Config settings as read from a .env file
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub firebase_project_id: String,
    pub redis_namespace: String,
    rocket_databases: String,
    pub cors_allow_origin: Option<String>,
    pub push_secret: String,
}

impl Config {
    pub fn redis_context(&self) -> Result<RedisPoolContext, redis::RedisError> {
        let db_toml = RedisConnToml::new(&self.rocket_databases).unwrap();
        let manager = rocket_contrib::databases::r2d2_redis::RedisConnectionManager::new(
            &db_toml.redis.url[..],
        )
        .unwrap();
        let pool = rocket_contrib::databases::r2d2::Pool::builder()
            .build(manager)
            .unwrap();
        Ok(RedisPoolContext {
            namespace: self.redis_namespace.to_string(),
            pool: pool,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RedisConnToml {
    pub redis: UrlToml,
}

impl RedisConnToml {
    /// Implemented the redis URL parsing using Regex since the TOML needed by
    /// Rocket is a fragment, and not a full TOML expression.  Parsing using the
    /// TOML lib is very ugly in this case.
    pub fn new(toml_s: &str) -> Result<RedisConnToml, ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"url\s*=\s*"(?P<url>\S+)""#).unwrap();
        }
        let caps: Option<_> = RE.captures(toml_s);
        if let Some(found) = caps {
            Ok(RedisConnToml {
                redis: {
                    UrlToml {
                        url: found["url"].to_string(),
                    }
                },
            })
        } else {
            Err(())
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UrlToml {
    pub url: String,
}

impl Config {
    pub fn new() -> Config {
        match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(e) => panic!("Unable to parse config ({})", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn regex_redis_toml() {
        let toml = r#"{ redis = { url = "redis://flipdrop:1111" } }"#;

        let actual: Result<RedisConnToml, _> = RedisConnToml::new(toml);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().redis.url, "redis://flipdrop:1111")
    }
}
