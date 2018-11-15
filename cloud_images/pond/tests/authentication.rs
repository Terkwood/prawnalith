#![feature(bind_by_move_pattern_guards)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

use pond::authentication::*;
use pond::claims::*;
use pond::key_pairs::{PubKey, SigningKeyId};

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

const PROJECT_ID: &str = "pond_test";

fn default_header_json() -> serde_json::value::Value {
    json! ({
        "alg": "RS256",
        "kid": *DEFAULT_SIGNING_KEY_ID,
    })
}

lazy_static! {
    static ref TEST_PUB_KEY: String = {
        let mut f = File::open("tests/public_key.pem").unwrap();

        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

        contents
    };
}

lazy_static! {
    static ref TEST_PRIV_KEY: String = {
        let mut f = File::open("tests/private_key.pem").unwrap();

        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

        contents
    };
}

lazy_static! {
    static ref DEFAULT_SIGNING_KEY_ID: SigningKeyId = SigningKeyId("test_rsa".to_string());
}

fn default_trusted_keys() -> HashMap<SigningKeyId, PubKey> {
    let mut h = HashMap::new();

    h.insert(
        DEFAULT_SIGNING_KEY_ID.clone(),
        PubKey(TEST_PUB_KEY.to_string()),
    );

    h
}

fn now() -> usize {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn earlier(offset: u64) -> usize {
    now() - offset as usize
}

fn later(offset: u64) -> usize {
    now() + offset as usize
}

fn default_claims() -> FirebaseClaims {
    FirebaseClaims {
        sub: SubjectClaim("SOMEBODY".to_string()),
        exp: later(3600),
        iat: earlier(3600),
        auth_time: earlier(3600),
        iss: format!("https://securetoken.google.com/{}", PROJECT_ID).to_string(),
        aud: PROJECT_ID.to_string(),
    }
}

#[test]
fn basic_encode_claims() {
    let claims = default_claims();

    let encoded = frank_jwt::encode(
        default_header_json(),
        &"secret".to_string(),
        &serde_json::to_value(claims).unwrap(),
        frank_jwt::Algorithm::RS256,
    );

    assert!(!encoded.is_ok())
}

#[test]
fn deserialize_claims() {
    let claims = r#"{
                            "sub": "abc",
                            "iss": "foogle",
                            "aud": "nope",
                            "iat": 1541969396,
                            "exp": 1541976596,
                            "auth_time": 1541969396
                        }"#;
    let deser: Result<FirebaseClaims, _> = serde_json::from_str(claims);
    assert!(deser.is_ok());
}

#[test]
fn authenticate_subject_claim() {
    let expect_valid = SubjectClaim("whoever".to_string()).authenticate();
    assert!(expect_valid)
}

#[test]
fn authenticate_happy_token() {
    let firebase_uid = "abcdefgh12345678ijklmnop90123456";
    let mut claims: FirebaseClaims = default_claims();
    claims.sub = SubjectClaim(firebase_uid.to_string());

    let token_r = frank_jwt::encode(
        default_header_json(),
        &*TEST_PRIV_KEY,
        &serde_json::to_value(claims).unwrap(),
        frank_jwt::Algorithm::RS256,
    );
    let token = token_r.unwrap();
    let result = authenticate(&token, default_trusted_keys(), PROJECT_ID);

    assert!(match result {
        AuthenticationResult::Valid(SubjectClaim(f)) if f == firebase_uid.to_string() => true,
        _ => false,
    })
}

#[test]
fn reject_empty_sub() {
    let mut claims: FirebaseClaims = default_claims();
    claims.sub = SubjectClaim("".to_string());

    let token_r = frank_jwt::encode(
        default_header_json(),
        &*TEST_PRIV_KEY,
        &serde_json::to_value(claims).unwrap(),
        frank_jwt::Algorithm::RS256,
    );
    let token = token_r.unwrap();
    let result = authenticate(&token, default_trusted_keys(), PROJECT_ID);

    assert!(match result {
        AuthenticationResult::Invalid(AuthenticationFailure::MissingSubject) => true,
        _ => false,
    })
}

#[test]
fn reject_auth_time_in_future() {
    let mut claims: FirebaseClaims = default_claims();
    claims.auth_time = later(3600);

    let token_r = frank_jwt::encode(
        default_header_json(),
        &*TEST_PRIV_KEY,
        &serde_json::to_value(claims).unwrap(),
        frank_jwt::Algorithm::RS256,
    );
    let token = token_r.unwrap();
    let result = authenticate(&token, default_trusted_keys(), PROJECT_ID);

    assert!(match result {
        AuthenticationResult::Invalid(AuthenticationFailure::AuthTime) => true,
        _ => false,
    })
}

#[test]
fn reject_exp_in_past() {
    let mut claims: FirebaseClaims = default_claims();
    claims.exp = earlier(3600);

    let token_r = frank_jwt::encode(
        default_header_json(),
        &*TEST_PRIV_KEY,
        &serde_json::to_value(claims).unwrap(),
        frank_jwt::Algorithm::RS256,
    );
    let token = token_r.unwrap();
    let result = authenticate(&token, default_trusted_keys(), PROJECT_ID);

    assert!(match result {
        AuthenticationResult::Invalid(AuthenticationFailure::Expired) => true,
        _ => false,
    })
}

#[test]
fn reject_iat_in_future() {
    let mut claims: FirebaseClaims = default_claims();
    claims.iat = later(3600);

    let token_r = frank_jwt::encode(
        default_header_json(),
        &*TEST_PRIV_KEY,
        &serde_json::to_value(claims).unwrap(),
        frank_jwt::Algorithm::RS256,
    );
    let token = token_r.unwrap();
    let result = authenticate(&token, default_trusted_keys(), PROJECT_ID);

    assert!(match result {
        AuthenticationResult::Invalid(AuthenticationFailure::IssuedAt) => true,
        _ => false,
    })
}

#[test]
fn reject_iss() {
    let mut claims: FirebaseClaims = default_claims();
    claims.iss = "some_path??///This Is Not The Project ID".to_string();

    let token_r = frank_jwt::encode(
        default_header_json(),
        &*TEST_PRIV_KEY,
        &serde_json::to_value(claims).unwrap(),
        frank_jwt::Algorithm::RS256,
    );
    let token = token_r.unwrap();
    let result = authenticate(&token, default_trusted_keys(), PROJECT_ID);

    assert!(match result {
        AuthenticationResult::Invalid(AuthenticationFailure::Issuer) => true,
        _ => false,
    })
}

#[test]
fn reject_aud() {
    let mut claims: FirebaseClaims = default_claims();
    claims.aud = "This Is Not The Project ID".to_string();

    let token_r = frank_jwt::encode(
        default_header_json(),
        &*TEST_PRIV_KEY,
        &serde_json::to_value(claims).unwrap(),
        frank_jwt::Algorithm::RS256,
    );
    let token = token_r.unwrap();
    let result = authenticate(&token, default_trusted_keys(), PROJECT_ID);

    assert!(match result {
        AuthenticationResult::Invalid(AuthenticationFailure::Audience) => true,
        _ => false,
    })
}

#[test]
fn reject_algo() {
    let claims: FirebaseClaims = default_claims();
    let header = json! ({
        "alg": "HS512",
        "kid": *DEFAULT_SIGNING_KEY_ID,
    });

    let token_r = frank_jwt::encode(
        header,
        &*TEST_PRIV_KEY,
        &serde_json::to_value(claims).unwrap(),
        frank_jwt::Algorithm::HS512,
    );
    let token = token_r.unwrap();
    let result = authenticate(&token, default_trusted_keys(), PROJECT_ID);

    assert!(match result {
        AuthenticationResult::Invalid(AuthenticationFailure::WrongAlgorithm(algo))
            if algo == "HS512".to_string() =>
        {
            true
        }
        _ => false,
    })
}
