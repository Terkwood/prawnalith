use hyper::net::HttpsConnector;
use redis_context::RedisContext;
use redis_delta::RDelta;
use yup_oauth2::GetToken;

use crate::model::{PubSubClient, PubSubContext};
use hyper_native_tls::NativeTlsClient;
use std::default::Default;

#[derive(Deserialize, Serialize, Debug)]
pub struct PubSubConfig {
    pub pubsub_project_id: Option<String>,
    pub pubsub_topic_name: Option<String>,
    pub pubsub_secret_file: Option<String>,
    pub redis_auth: Option<String>,
    pub redis_host: Option<String>,
    pub redis_port: Option<u16>,
    pub redis_namespace: Option<String>,
}

impl PubSubConfig {
    pub fn new() -> PubSubConfig {
        match envy::from_env::<PubSubConfig>() {
            Ok(config) => config,
            Err(e) => panic!("Unable to parse config ({})", e),
        }
    }

    /// Create an object which holds both a connection to redis
    /// and a string "namespace" used to prefix all keys.
    pub fn to_redis_context(&self) -> RedisContext {
        RedisContext::new(
            (self.redis_host.clone().unwrap_or("127.0.0.1".to_string())).to_string(),
            self.redis_port.unwrap_or(6379),
            self.redis_auth.clone(),
            self.redis_namespace.clone().unwrap_or("".to_string()),
        )
    }

    /// Create a client used to publish to google pub/sub.
    /// See instructions at https://docs.rs/google-pubsub1_beta2/1.0.8+20181001/google_pubsub1_beta2/
    pub fn to_pubsub_client(&self) -> PubSubClient {
        let secret_file = self
            .pubsub_secret_file
            .clone()
            .unwrap_or("secret.json".to_string());
        let client_secret =
            yup_oauth2::service_account_key_from_file(&"pubsub-auth.json".to_string()).unwrap();
        let client =
            hyper::Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
        let mut access = yup_oauth2::ServiceAccountAccess::new(client_secret, client);

        println!(
            "{:?}",
            access
                .token(&vec!["https://www.googleapis.com/auth/pubsub"])
                .unwrap()
        );

        let client =
            hyper::Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
        google_pubsub1::Pubsub::new(client, access)
    }

    pub fn to_pubsub_context(&self) -> PubSubContext {
        let project_id = self
            .pubsub_project_id
            .clone()
            .unwrap_or("project".to_string());
        let topic_name = self
            .pubsub_topic_name
            .clone()
            .unwrap_or("topic".to_string());
        let fq_topic = format!("projects/{}/topics/{}", project_id, topic_name);
        let client = self.to_pubsub_client();
        PubSubContext { fq_topic, client }
    }
}
