extern crate dotenv;
extern crate envy;
extern crate redis;
extern crate redis_aggregator;
extern crate redis_context;
extern crate uuid;

use google_pubsub1::PublishRequest;
use redis_aggregator::config::PubSubConfig;
use redis_aggregator::pubsub::PubSubContext;
use redis_context::RedisContext;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = PubSubConfig::new();

    let redis_ctx = &config.to_redis_context();
    let pubsub_ctx = &config.to_pubsub_context();

    hello_world(redis_ctx, pubsub_ctx);
}

fn hello_world(_redis_ctx: &RedisContext, pubsub_ctx: &PubSubContext) {
    let message = google_pubsub1::PubsubMessage {
        // Base64 encoded!
        data: Some(base64::encode("HELLO ANYONE!".as_bytes())),
        ..Default::default()
    };

    let req = PublishRequest {
        messages: Some(vec![message]),
    };

    println!("Publishing to {}", &pubsub_ctx.fq_topic);
    pubsub_ctx
        .client
        .projects()
        .topics_publish(req, &pubsub_ctx.fq_topic)
        .doit()
        .unwrap();
}
