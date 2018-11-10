extern crate crossbeam_channel;
extern crate dotenv;
extern crate envy;
extern crate redis;
extern crate redis_aggregator;
extern crate redis_context;
extern crate uuid;

use redis_aggregator::config::PubSubConfig;
use redis_aggregator::{clone_the_world, consume_redis_messages, handle_revents};

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = PubSubConfig::new();

    println!("Cloning database...");

    clone_the_world(&config).unwrap();

    let (tx, rx) = crossbeam_channel::unbounded();

    std::thread::spawn(move || handle_revents(rx, &config));

    consume_redis_messages(&PubSubConfig::new(), tx)
}
