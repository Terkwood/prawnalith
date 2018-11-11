extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;

use self::jwt::{decode, encode, Algorithm, Header, Validation};
use std::time::{Duration, SystemTime};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn main() {
    let my_claims = Claims {
        sub: "Monger".to_string(),
        company: "Norporate".to_string(),
        exp: the_future(),
    };
    let token = encode(&Header::default(), &my_claims, "secret".as_ref()).unwrap();
}

fn the_future() -> usize {
    let r = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;
    r as usize
}
