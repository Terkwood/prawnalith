extern crate dotenv;
extern crate envy;
extern crate redis;
extern crate redis_aggregator;
extern crate redis_context;
extern crate uuid;

use redis_aggregator::config::PubSubConfig;
use redis_aggregator::hello_world;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = PubSubConfig::new();

    let redis_ctx = &config.to_redis_context();
    let pubsub_ctx = &config.to_pubsub_context();

    hello_world(redis_ctx, pubsub_ctx);
}
