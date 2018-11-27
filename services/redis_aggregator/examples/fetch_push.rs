extern crate dotenv;
extern crate envy;
extern crate redis;
extern crate redis_aggregator;
extern crate redis_context;
extern crate uuid;

use redis_aggregator::config::PubSubConfig;
use redis_aggregator::publish_recent;
use redis_delta::REvent;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = PubSubConfig::new();

    let redis_ctx = &config.to_redis_context();
    let pubsub_ctx = &config.to_pubsub_context();

    publish_recent(
        redis_ctx,
        pubsub_ctx,
        vec![REvent::StringUpdated {
            key: format!("{}/tanks", redis_ctx.namespace).to_string(),
        }],
    )
    .unwrap();

    publish_recent(
        redis_ctx,
        pubsub_ctx,
        vec![REvent::HashUpdated {
            key: format!("{}/tanks/1", redis_ctx.namespace).to_string(),
            fields: vec!["name".to_string()],
        }],
    )
    .unwrap();
}
