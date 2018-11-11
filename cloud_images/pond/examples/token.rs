// LET THE ATTRIBUTION BE KNOWN!
// This is basically the same as what you see on https://github.com/Keats/jsonwebtoken/blob/master/README.md

extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;

use self::jwt::{decode, decode_header, encode, Algorithm, Header, Validation};
use std::time::SystemTime;

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
    let there = encode(&Header::default(), &my_claims, "secret".as_ref()).unwrap();
    let _back = decode::<Claims>(&there, "secret".as_ref(), &Validation::default()).unwrap();

    // In some cases, for example if you don't know the algorithm used, you will want to only decode the header:
    let _header = decode_header(&there).unwrap();
}

fn the_future() -> usize {
    let r = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;
    r as usize
}
