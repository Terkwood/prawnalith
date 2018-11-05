//! # gcloud_push
//!
//! This is a service which pushes temperature and pH
//! data to google cloud pub sub.  The temp & pH
//! data is expected to reside in a Redis instance.
#![feature(custom_attribute)]
extern crate google_pubsub1_beta2 as pubsub;
extern crate hyper;
extern crate hyper_rustls;
extern crate redis_context;
#[macro_use]
extern crate serde_derive;
extern crate yup_oauth2;

use redis_context::RedisContext;
use redis_delta::RDelta;

use self::pubsub::Pubsub;
use self::pubsub::{Error, Result};
use self::pubsub::{PublishRequest, PubsubMessage};
use std::default::Default;
use yup_oauth2::{ApplicationSecret, Authenticator, DefaultAuthenticatorDelegate, MemoryStorage};

pub fn hello_world(redis_ctx: &RedisContext, pubsub_ctx: &PubSubContext) {
    let req = PublishRequest {
        messages: Some(vec![PubsubMessage::default()]),
    };
    pubsub_ctx
        .client
        .projects()
        .topics_publish(req, &pubsub_ctx.fq_topic);
}

/// send *all* relevant redis data upstream
/// to the cloud instance via pub sub
///
/// # How to do this
///
/// - Figure out how many tanks there are
/// - Query each tank individually
/// - Figure out what types of sensors there are
/// - For each type of sensor, query for all the sensor IDs
/// - Query each individual sensor of each type
///
/// Push as you satisfy each individual step.
pub fn clone_the_world(redis_ctx: &RedisContext, pubsub_ctx: &PubSubContext) {
    unimplemented!()
}

/// pushes some recent data via gcloud pubsub
pub fn push_recent(
    redis_context: &RedisContext,
    pubsub: &PubSubClient,
    rdeltas: Vec<RDelta>,
) -> Result<()> {
    unimplemented!()
}

/// Note that fq_topic is a fully qualified topic, i.e. `projects/{project_id}/topics/{topic_name}`
pub struct PubSubContext {
    pub fq_topic: String,
    pub client: PubSubClient,
}

pub type PubSubClient = pubsub::Pubsub<
    hyper::Client,
    yup_oauth2::Authenticator<
        yup_oauth2::DefaultAuthenticatorDelegate,
        yup_oauth2::MemoryStorage,
        hyper::Client,
    >,
>;

#[derive(Deserialize, Serialize, Debug)]
pub struct PubSubConfig {
    pub pubsub_project_id: Option<String>,
    pub pubsub_topic_name: Option<String>,
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
        let secret: ApplicationSecret = Default::default();
        let auth = Authenticator::new(
            &secret,
            DefaultAuthenticatorDelegate,
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            <MemoryStorage as Default>::default(),
            None,
        );
        Pubsub::new(
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            auth,
        )
    }

    pub fn to_pubsub_context(&self) -> PubSubContext {
        let project_id = self
            .pubsub_project_id
            .clone()
            .unwrap_or("project".to_string());
        let fq_topic = format!(
            "projects/{}/topics/{}",
            project_id,
            self.pubsub_topic_name
                .clone()
                .unwrap_or("topic".to_string())
        );
        PubSubContext {
            fq_topic,
            client: self.to_pubsub_client(),
        }
    }
}
