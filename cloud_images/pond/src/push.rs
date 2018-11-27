use base64;
use crate::redis_conn::RedisDbConn;
use crypto::hmac::Hmac;
use crypto::mac::{Mac, MacResult};
use crypto::sha3::Sha3;
use redis_delta::RDelta;
use rocket_contrib::databases::redis::Commands;
use std::collections::HashMap;

/// Push data structure which adheres to Google Cloud Pub/Sub
/// specification.  Each of these is an individual Redis Delta
/// containing various payloads depending on the Redis type
/// that it represents.
/// See https://cloud.google.com/pubsub/docs/push
#[derive(Debug, Deserialize)]
pub struct PushData {
    pub message: Message,
    pub subscription: String,
}

impl PushData {
    // Note that we aren't checking the order of messages.
    pub fn ingest(&self, conn: RedisDbConn) -> Result<(), PushDataError> {
        let rdelta = self.message.deserialize()?;
        let result = match rdelta {
            RDelta::UpdateHash {
                key,
                mut fields,
                time: _,
            } => {
                let mut name_vals: Vec<(String, String)> = vec![];
                for rf in fields.drain(..) {
                    name_vals.push((rf.name, rf.val));
                }
                Ok(conn.0.hset_multiple(key, &name_vals)?)
            }
            RDelta::UpdateSet { key, vals, time: _ } => Ok(conn.0.sadd(key, vals)?),
            RDelta::UpdateString { key, val, time: _ } => Ok(conn.0.set(key, val)?),
        };

        if let Err(e) = &result {
            eprintln!("Error on ingest! {:?}", e)
        }

        result
    }
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub attributes: Option<HashMap<String, String>>,
    pub data: Base64,
    pub message_id: String,
}
impl Message {
    pub fn deserialize(&self) -> Result<RDelta, PushDataError> {
        let json_r: Result<RDelta, _> = serde_json::from_slice(&self.data.decode()?[..]);
        Ok(json_r?)
    }

    /// Verify that this message payload is sent by our redis aggregator.
    pub fn verify_signature(&self, secret: &[u8]) -> bool {
        if let Some(attrs) = &self.attributes {
            if let Some(mac) = attrs.get("sig") {
                if let Ok(sig_bytes) = base64::decode(mac) {
                    // comparison using MacResult should be quick
                    return sign(&self.data.0, secret) == MacResult::new(&sig_bytes);
                }
            }
        }

        false
    }
}

/// Provides a base64-encoded hmac signature for the base64
/// message being sent.
fn sign(message_base64: &str, secret: &[u8]) -> MacResult {
    // create a SHA3-256 object
    let mut hmac = Hmac::new(Sha3::sha3_256(), secret);

    hmac.input(message_base64.as_bytes());

    hmac.result()
}

#[derive(Debug, Deserialize)]
pub struct Base64(pub String);
impl Base64 {
    /// You can consume this with `serde_json::from_slice`
    pub fn decode(&self) -> Result<Vec<u8>, base64::DecodeError> {
        base64::decode(&self.0)
    }
}

#[derive(Debug)]
pub enum PushDataError {
    Base64,
    Json,
    Utf8,
    Redis,
}
impl From<rocket_contrib::databases::redis::RedisError> for PushDataError {
    fn from(_e: rocket_contrib::databases::redis::RedisError) -> PushDataError {
        PushDataError::Redis
    }
}
impl From<std::str::Utf8Error> for PushDataError {
    fn from(_e: std::str::Utf8Error) -> PushDataError {
        PushDataError::Utf8
    }
}
impl From<serde_json::Error> for PushDataError {
    fn from(_e: serde_json::Error) -> PushDataError {
        PushDataError::Json
    }
}

impl From<base64::DecodeError> for PushDataError {
    fn from(_e: base64::DecodeError) -> PushDataError {
        PushDataError::Base64
    }
}
