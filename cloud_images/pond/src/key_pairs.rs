use crate::redis_conn::{RedisDbConn, RedisPoolContext};
use openssl::x509::X509;
use reqwest;
use rocket_contrib::databases::redis::Commands;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

const GOOGLE_PUBLIC_KEY_URL: &'static str =
    "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

/// The key ID references a public certificate
/// from https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct SigningKeyId(pub String);

/// This string is expected to include the header and footer
/// seen in public certificate PEM files, e.g.
/// ```ignore
/// -----BEGIN CERTIFICATE-----
/// MIIDHDCCAgSgAwIBAgIIGLZwido7rJUwDQYJKoZIhvcNAQEFBQAwMTEvMC0GA1UE
/// AxMmc2VjdXJldG9rZW4uc3lzdGVtLmdzZXJ2aWNlYWNjb3VudC5jb20wHhcNMTgx
/// MTA5MjEyMDQzWhcNMTgxMTI2MDkzNTQzWjAxMS8wLQYDVQQDEyZzZWN1cmV0b2tl
/// bi5zeXN0ZW0uZ3NlcnZpY2VhY2NvdW50LmNvbTCCASIwDQYJKoZIhvcNAQEBBQAD
/// ...                         more                             ...
/// KoZIhvcNAQEFBQADggEBAA1PqG5IUb8jfftyvL6sGrHI7IzH3w3fncv/T/pAOTyt
/// 71JCDI9LBseZ9vFeZqtxX4LalbT4aZW2/wTtdPHpb5TxtWX3gW9umu0xM3n0WSjW
/// PqPOXLvmLYgmCt3ZVrxhhQ+O/ouxO8RxLmktjGPunCoB/DHARfmj+t5jVTR1Jmv5
/// cQkuiby96FkEIB8sUlVXu6it91PxeJOlecFSUBBJiI+CcLfkOLgDSkVJvoN9W0dq
/// XA2bqiND10dZk8X412YRViRLbXjpPboH0eCAQBJR1ZT3r2BBBMtYlw2FLd+Tn0LT
/// GBuGIURzpq5Ukj1he/bNxcimcO6sDE7WRqacbEFrbUo=
/// -----END CERTIFICATE-----
/// ```
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct PubCert(pub String);
impl PubCert {
    pub fn to_pub_key(&self) -> Result<PubKey, PubCertErr> {
        let cert = X509::from_pem(self.0.as_bytes())?;
        let pub_key_pem = cert.public_key()?.public_key_to_pem()?;
        Ok(PubKey(std::str::from_utf8(&pub_key_pem[..])?.to_string()))
    }
}
#[derive(Debug)]
pub enum PubCertErr {
    OpenSSLErr(openssl::error::ErrorStack),
    PemConvertErr(std::str::Utf8Error),
}

impl From<openssl::error::ErrorStack> for PubCertErr {
    fn from(e: openssl::error::ErrorStack) -> PubCertErr {
        PubCertErr::OpenSSLErr(e)
    }
}
impl From<std::str::Utf8Error> for PubCertErr {
    fn from(e: std::str::Utf8Error) -> PubCertErr {
        PubCertErr::PemConvertErr(e)
    }
}

/// This string is expected to include the header and footer
/// seen in RSA public key PEM files, e.g.
/// ```ignore
/// -----BEGIN PUBLIC KEY-----
/// MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAuFMd9UG9RoPVNer2u19o
/// Rf9sMh1ge3MUqwwSK3xc4xHGkoILXDgzc6pY5AUl7Mmvw9+KvAENd5HXmRLWtEy6
/// ...                         more                             ...
/// CQYbwIUf5OZfDjnoOdbU6EJJpyLf36vbjTsz/Rl7yP1u7REfh8kq+19K1UqDrzPN
/// aQIDAQAB
/// -----END PUBLIC KEY-----
/// ```
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct PubKey(pub String);

fn redis_key_for_rsa_pub_key(namespace: &str) -> String {
    let fragment: &str = "pond/firebase/public_signing_keys";
    format!("{}/{}", namespace, fragment)
}

pub fn fetch_from_redis(
    conn: &RedisDbConn,
    namespace: &str,
) -> Result<HashMap<SigningKeyId, PubKey>, rocket_contrib::databases::redis::RedisError> {
    let r: HashMap<String, String> = conn.hgetall(redis_key_for_rsa_pub_key(namespace))?;
    Ok(r.iter()
        .map(|(k, v)| (SigningKeyId(k.to_string()), PubKey(v.to_string())))
        .collect())
}

/// Grabs the public keys from
/// https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com
/// Uses the value of max-age in the Cache-Control header of the response from that endpoint
/// to know when to refresh the public keys.
pub fn refresh_loop(redis_context: &RedisPoolContext) {
    static WAIT_ON_FAIL: u64 = 10;
    loop {
        if let Some(pub_cert_result) = fetch_from_web().ok() {
            let redis_key = redis_key_for_rsa_pub_key(&redis_context.namespace);

            // Convert PEM certs to PEM keys.
            let maybe_pub_keys: Vec<(&SigningKeyId, Option<PubKey>)> = pub_cert_result
                .payload
                .iter()
                .map(|(kid, cert)| {
                    let pc = PubCert(cert.0.to_string());
                    (kid, pc.to_pub_key().ok())
                })
                .collect();

            for (kid, maybe_pub_key) in &maybe_pub_keys {
                if let None = maybe_pub_key {
                    eprintln!(
                        "Unable to convert key {:?} from cert PEM to pub key PEM",
                        kid
                    );
                }
            }

            let pub_keys: Vec<(String, String)> = maybe_pub_keys
                .iter()
                .filter(|(_, v)| v.is_some())
                .map(|(kid, pub_key)| (kid.0.to_string(), pub_key.clone().unwrap().0))
                .collect();

            let written: Result<(), _> = redis_context
                .pool
                .get()
                .unwrap()
                .hset_multiple(redis_key, &pub_keys[..]);
            if let Err(e) = written {
                eprintln!("Couldn't write public signing keys to Redis: {:?}", e);
            }

            thread::sleep(Duration::from_secs(pub_cert_result.max_age as u64));
        } else {
            // The http fetch failed, so we need to wait some arbitrary
            // amount of time before trying again.
            eprintln!(
                "Failed to fetch the signing keys.  Retrying in {} seconds...",
                WAIT_ON_FAIL
            );
            thread::sleep(Duration::from_secs(WAIT_ON_FAIL));
        }
    }
}

/// Contains the public signing keys used by Google Firebase
/// to sign JWTs.  Also provides the `max_age` variable,
/// which tells us when we need to refresh the keys.
struct WebPublicCertResult {
    pub payload: HashMap<SigningKeyId, PubCert>,
    pub max_age: u32,
}
const MAX_AGE_DEFAULT_SECS: u32 = 3600;
/// Fetches public signing keys used by Google Firebase.
/// This function will trim away the -BEGIN CERT- and -END CERT-
/// wrapper around the payload, and only return the base64 payload
/// for each key.
/// Per `WebPublicSigningKeyResult`, this function also returns
/// the `max_age`, which tells us when we'll need to refresh the
/// keys.
fn fetch_from_web() -> Result<WebPublicCertResult, reqwest::Error> {
    let mut response = reqwest::get(GOOGLE_PUBLIC_KEY_URL)?;
    let max_age_i = response.headers().get("max_age");
    let max_age: u32 = match max_age_i {
        None => MAX_AGE_DEFAULT_SECS,
        Some(header_val) => {
            if let Some(s) = header_val.to_str().ok() {
                str::parse::<u32>(s).unwrap_or(MAX_AGE_DEFAULT_SECS)
            } else {
                MAX_AGE_DEFAULT_SECS
            }
        }
    };
    Ok(WebPublicCertResult {
        payload: response.json()?,
        max_age,
    })
}
