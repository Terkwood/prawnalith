//! # Redis Aggregator
//!
//! This is a service which monitors changes in
//! prawn-related data stored in our Redis database.
//! It keeps track of changed keys, and periodically
//! queries Redis for the values related to those keys,
//! then pushing the values via Google pub sub.
#![feature(custom_attribute)]
extern crate google_pubsub1;
extern crate hyper;
extern crate hyper_native_tls;
extern crate redis_context;
#[macro_use]
extern crate serde_derive;
extern crate yup_oauth2;

pub mod config;
pub mod pubsub;

use base64;
use redis::Commands;
use redis_context::RedisContext;
use redis_delta::{Key, RDelta, REvent, RField};

use self::pubsub::PubSubContext;
use std::default::Default;
use std::time::SystemTime;
use uuid::Uuid;

/// Send *all* relevant redis data upstream
/// to the cloud instance via pub sub.
///
/// This is expected to be invoked on service startup.
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
pub fn clone_the_world(redis_ctx: &RedisContext, pubsub_ctx: &PubSubContext) -> Result<(), AggErr> {
    let all_ids: Vec<REvent> = instantiate_all_ids(redis_ctx)?;

    push_recent(redis_ctx, pubsub_ctx, all_ids)
}

#[derive(Debug)]
pub enum AggErr {
    Redis(redis::RedisError),
    PubSub,
}

impl From<redis::RedisError> for AggErr {
    fn from(error: redis::RedisError) -> Self {
        AggErr::Redis(error)
    }
}
impl From<google_pubsub1::Error> for AggErr {
    fn from(_error: google_pubsub1::Error) -> Self {
        AggErr::PubSub
    }
}

fn instantiate_all_ids(redis_ctx: &RedisContext) -> Result<Vec<REvent>, redis::RedisError> {
    let mut result: Vec<REvent> = vec![];

    let ns = redis_delta::Namespace(&redis_ctx.namespace);

    let all_tanks_key = Key::AllTanks { ns }.to_string();

    let maybe_num_tanks: Option<u16> = redis_ctx.conn.get(&all_tanks_key)?;

    if let Some(num_tanks) = maybe_num_tanks {
        // We know that there's an entry describing the number of
        // tanks in the system, so we'll return the ID.
        let all_tanks_event = REvent::StringUpdated { key: all_tanks_key };
        result.push(all_tanks_event);

        // We should see if there are hash entries for the individual tanks.
        // Use https://redis.io/commands/hkeys to look up all field names
        // for each tank that we find.
        let each_tank_events: Result<Vec<REvent>, redis::RedisError> =
            tank_hash_events(num_tanks, redis_ctx);

        for e in each_tank_events? {
            result.push(e);
        }
    }

    let sensor_types_key = Key::AllSensorTypes { ns }.to_string();
    let sensor_type_members: Vec<String> = redis_ctx.conn.smembers(&sensor_types_key)?;
    if sensor_type_members.is_empty() {
        result.push(REvent::SetUpdated {
            key: sensor_types_key,
        })
    }

    for sensor_type in sensor_type_members {
        let st = redis_delta::SensorType(&sensor_type);

        // look up each "all temp sensors", "all ph sensors" set
        let all_sensors_key = Key::AllSensors { ns, st }.to_string();
        let all_sensors_members: Vec<String> = redis_ctx.conn.smembers(&all_sensors_key)?;
        if all_sensors_members.len() > 0 {
            result.push(REvent::SetUpdated {
                key: all_sensors_key,
            })
        }

        // deal with each individual sensor hash
        for e in sensor_hash_events(st, all_sensors_members, redis_ctx)? {
            result.push(e)
        }
    }

    Ok(result)
}

fn sensor_hash_events(
    st: redis_delta::SensorType,
    ids: Vec<String>,
    redis_ctx: &RedisContext,
) -> Result<Vec<REvent>, redis::RedisError> {
    let mut r: Vec<REvent> = vec![];
    for id in ids {
        let key = Key::Sensor {
            ns: redis_delta::Namespace(&redis_ctx.namespace),
            st,
            id: Uuid::parse_str(&id).unwrap(),
        }
        .to_string();
        for hash in hash_event(&key, redis_ctx)? {
            r.push(hash)
        }
    }
    Ok(r)
}

fn tank_hash_events(
    num_tanks: u16,
    redis_ctx: &RedisContext,
) -> Result<Vec<REvent>, redis::RedisError> {
    let mut r: Vec<REvent> = vec![];
    for id in 1..=num_tanks {
        let key = Key::Tank {
            ns: redis_delta::Namespace(&redis_ctx.namespace),
            id,
        }
        .to_string();
        for hash in hash_event(&key, redis_ctx)? {
            r.push(hash)
        }
    }

    Ok(r)
}

fn hash_event(key: &str, redis_ctx: &RedisContext) -> Result<Option<REvent>, redis::RedisError> {
    let fields: Vec<String> = redis_ctx.conn.hkeys(key)?;
    if fields.is_empty() {
        Ok(None)
    } else {
        Ok(Some(REvent::HashUpdated {
            key: key.to_string(),
            fields,
        }))
    }
}

/// Publish a vec of redis changes (hash updates, string updates, etc)
/// to google pubsub system.
///
/// In order to get this done, we need to first retrieve a recent
/// copy of each piece of data referred to in the vec of events.
/// - For string or set updates, this simply means retrieving the
///   entire set, or the string itself.
/// - For hash field updates, we only retrieve the fields
///   which have been updated.
pub fn push_recent<'a, 'b, 'c>(
    redis_ctx: &'a RedisContext,
    pubsub_ctx: &'b PubSubContext,
    redis_events: Vec<REvent>,
) -> Result<(), AggErr> {
    let them: Vec<Result<(), google_pubsub1::Error>> = redis_events
        .iter()
        .map(|revent| {
            let fetched = fetch(revent, redis_ctx);
            fetched.map(|found| found.map(|f| push(&f, pubsub_ctx)))
        })
        .flatten()
        .filter(|maybe| maybe.is_some())
        .map(|some| some.unwrap())
        .collect();

    if them.iter().any(|i| i.is_err()) {
        Err(AggErr::PubSub)
    } else {
        Ok(())
    }
}

fn fetch<'a>(
    event: &'a REvent,
    ctx: &RedisContext,
) -> Result<Option<RDelta<'a>>, redis::RedisError> {
    match event {
        REvent::HashUpdated { key, fields } => {
            fetch_hash_delta(key, fields.to_vec(), ctx).map(|r| Some(r))
        }
        REvent::StringUpdated { key } => fetch_string_delta(key, ctx),
        REvent::SetUpdated { key } => fetch_set_delta(key, ctx),
    }
}

fn fetch_string_delta<'a>(
    key: &'a str,
    ctx: &RedisContext,
) -> Result<Option<RDelta<'a>>, redis::RedisError> {
    let found: Option<String> = ctx.conn.get(key)?;
    Ok(found.map(|f| RDelta::UpdateString {
        key,
        val: f,
        time: epoch_secs(),
    }))
}

fn fetch_set_delta<'a>(
    key: &'a str,
    ctx: &RedisContext,
) -> Result<Option<RDelta<'a>>, redis::RedisError> {
    let found: Option<Vec<String>> = ctx.conn.smembers(key)?;
    Ok(found.map(|f| RDelta::UpdateSet {
        key,
        vals: f,
        time: epoch_secs(),
    }))
}

fn fetch_hash_delta<'a>(
    key: &'a str,
    fields: Vec<String>,
    ctx: &RedisContext,
) -> Result<RDelta<'a>, redis::RedisError> {
    let fields_forever = fields.clone();
    let found: Vec<Option<String>> = ctx.conn.hget(key, fields)?;
    let zipped = fields_forever.iter().zip(found);
    let rfields: Vec<RField> = zipped
        .map(|(f, maybe_v)| {
            maybe_v.map(|v| RField {
                name: f.to_string(),
                val: v,
            })
        })
        .filter(|maybe| maybe.is_some())
        .map(|some| some.unwrap())
        .collect();

    Ok(RDelta::UpdateHash {
        key,
        fields: rfields,
        time: epoch_secs(),
    })
}

fn push(data: &RDelta, pubsub_ctx: &PubSubContext) -> Result<(), google_pubsub1::Error> {
    let message = google_pubsub1::PubsubMessage {
        // This must be base64 encoded!
        data: Some(base64::encode(
            serde_json::to_string(data).unwrap().as_bytes(),
        )),
        ..Default::default()
    };

    let req = google_pubsub1::PublishRequest {
        messages: Some(vec![message]),
    };

    pubsub_ctx
        .client
        .projects()
        .topics_publish(req, &pubsub_ctx.fq_topic)
        .doit()
        .map(|_r| ())
}

fn epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Consume all outstanding messages from a pubsub connection.
pub fn consume_redis_messages(
    topic: &str,
    redis_client: redis::Client,
) -> Result<Vec<REvent>, redis::RedisError> {
    // FIXME tediously move initialization
    // FIXME and deal with glorious borrow
    let mut sub_conn = redis_client.get_connection()?;
    let mut sub = sub_conn.as_pubsub();
    sub.subscribe(topic)?;

    let mut result: Vec<REvent> = vec![];

    while let Ok(msg) = sub.get_message() {
        unimplemented!()
    }

    Ok(result)
}
