use base64;
use crate::claims::*;
use crate::key_pairs::{PubKey, SigningKeyId};
use frank_jwt;
use std::collections::HashMap;

/// Decodes a JWT. If the token or its signature is invalid or
/// the claims fail validation, it will return an error.
/// The JWT's kid header is expected to match one of the keys
/// in `key_pairs`.
fn decode(
    token: &str,
    key_pairs: HashMap<SigningKeyId, PubKey>,
) -> Result<FirebaseClaims, AuthenticationFailure> {
    let headers: JwtHeader = decode_header(token)?;

    if headers.kid.is_empty() {
        return Err(AuthenticationFailure::KidHeaderMissing);
    }

    if headers.alg.trim().to_ascii_uppercase() != "RS256" {
        return Err(AuthenticationFailure::WrongAlgorithm(headers.alg));
    }

    // check that one of the known key IDs was used to sign the token
    let maybe_key = key_pairs
        .iter()
        .find(|(SigningKeyId(kid), _)| kid == &headers.kid)
        .map(|(_, key)| key);

    let pub_key_pem: &PubKey = match maybe_key {
        None => Err(AuthenticationFailure::KidHeaderNoMatch),
        Some(k) => Ok(k),
    }?;

    let decode_result = frank_jwt::decode(
        &token.to_string(),
        &pub_key_pem.0,
        frank_jwt::Algorithm::RS256,
    );

    let payload_json = match decode_result {
        Err(e) => Err(AuthenticationFailure::PayloadDecoding(e)),
        Ok((_header, payload)) => Ok(payload),
    }?;

    Ok(serde_json::from_value(payload_json)?)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JwtHeader {
    pub alg: String,
    pub kid: String,
}

/// Authenticate a user based on a given JWT.
/// See https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library
/// The key pairs you need will stored in redis.
pub fn authenticate(
    encoded_token: &str,
    key_pairs: HashMap<SigningKeyId, PubKey>,
    firebase_project_id: &str,
) -> AuthenticationResult {
    match decode(encoded_token, key_pairs) {
        Err(auth_failure) => AuthenticationResult::Invalid(auth_failure),
        Ok(claims) => claims.validate(firebase_project_id),
    }
}

/// Describes whether authorization was successfuly, and if it wasn't, describes why it failed.
#[derive(Debug)]
pub enum AuthenticationResult {
    Valid(SubjectClaim),
    Invalid(AuthenticationFailure),
}

/// Data structure representing why authorization failed:
///
/// - `JwtValidation`: one of many possible JWT validation errors described in https://github.com/Keats/jsonwebtoken/blob/master/src/errors.rs
/// - `KidHeader`: key ID header doesn't correspond to a public key listed by google
/// - `AuthTimeClaim`: claim value is not in the past
/// - `Unknown`:
#[derive(Debug)]
pub enum AuthenticationFailure {
    WrongAlgorithm(String),
    PayloadDecoding(frank_jwt::error::Error),
    HeaderDecoding(HeaderDecodeErr),
    ClaimsDeserialization(serde_json::Error),
    MissingSubject,
    KidHeaderMissing,
    KidHeaderNoMatch,
    AuthTime,
    Expired,
    Audience,
    IssuedAt,
    Issuer,
}

impl From<frank_jwt::error::Error> for AuthenticationFailure {
    fn from(error: frank_jwt::error::Error) -> AuthenticationFailure {
        AuthenticationFailure::PayloadDecoding(error)
    }
}

impl From<serde_json::Error> for AuthenticationFailure {
    fn from(error: serde_json::Error) -> AuthenticationFailure {
        AuthenticationFailure::ClaimsDeserialization(error)
    }
}
impl From<HeaderDecodeErr> for AuthenticationFailure {
    fn from(error: HeaderDecodeErr) -> AuthenticationFailure {
        AuthenticationFailure::HeaderDecoding(error)
    }
}

pub fn decode_header(encoded_token: &str) -> Result<JwtHeader, HeaderDecodeErr> {
    const SEGMENTS_COUNT: usize = 3;

    let raw_segments: Vec<&str> = encoded_token.split(".").collect();
    let num_segs = raw_segments.len();
    if num_segs != SEGMENTS_COUNT {
        return Err(HeaderDecodeErr::NumberOfSegments(num_segs));
    }

    let header_segment = raw_segments[0];
    let header_json = serde_json::from_slice(
        base64::decode_config(header_segment, base64::URL_SAFE_NO_PAD)?.as_slice(),
    )?;

    Ok(serde_json::from_value(header_json)?)
}
#[derive(Debug)]
pub enum HeaderDecodeErr {
    NumberOfSegments(usize),
    Base64(base64::DecodeError),
    Json(serde_json::Error),
}
impl From<serde_json::Error> for HeaderDecodeErr {
    fn from(e: serde_json::Error) -> HeaderDecodeErr {
        HeaderDecodeErr::Json(e)
    }
}

impl From<base64::DecodeError> for HeaderDecodeErr {
    fn from(e: base64::DecodeError) -> HeaderDecodeErr {
        HeaderDecodeErr::Base64(e)
    }
}
