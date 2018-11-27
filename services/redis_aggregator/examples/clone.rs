extern crate dotenv;
extern crate envy;
extern crate redis;
extern crate redis_aggregator;
extern crate redis_context;
extern crate uuid;

use redis_aggregator::clone_the_world;
use redis_aggregator::config::PubSubConfig;
use redis_delta::REvent;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = PubSubConfig::new();

    println!("Wherein We Clone the World");

    clone_the_world(&config);
}
