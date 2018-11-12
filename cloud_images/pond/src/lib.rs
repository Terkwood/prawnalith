extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use self::jwt::{Algorithm, Validation};

/// JWT claims structure as defined by Firebase documentation.
/// See https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library
/// Summary of the field definitions is mostly taken from that page.
///
/// ## Claims summary
/// - `sub`: Must be a non-empty string and must be the uid of the user or device. Ultimately, pond service will restrict this to a hallowed group of users.
/// - `iss`: Must be `"https://securetoken.google.com/<projectId>"`, where `<projectId>` is the same project ID used for `aud`.  
/// - `aud`: Must be your Firebase project ID, the unique identifier for your Firebase project, which can be found in the URL of that project's console.
/// - `iat`: Must be in the past. The time is measured in seconds since the UNIX epoch. Validated automatically by `jsonwebtoken`.
/// - `exp`: Must be in the future. The time is measured in seconds since the UNIX epoch. Validated automatically by `jsonwebtoken`.
/// - `auth_time`: Must be in the past. The time when the user authenticated.
#[derive(Debug, Serialize, Deserialize)]
struct FirebaseClaims {
    sub: SubjectClaim,
    iss: String,
    aud: String,
    iat: usize,
    exp: usize,
    auth_time: usize,
}

/// Must be a non-empty string and must be the uid of the user or device
#[derive(Debug, Serialize, Deserialize)]
struct SubjectClaim(String);
impl SubjectClaim {
    fn validate(&self, allowed_uids: Vec<&str>) -> bool {
        !self.0.is_empty() && allowed_uids.iter().any(|uid| uid == &self.0)
    }
}

/// Creates a Validation struct which conforms to Firebase auth expectations.
/// See See https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library
fn validate_token<E>(token: jwt::TokenData<FirebaseClaims>) -> Result<AuthResult, E> {
    // TODO: VERIFY HEADER `alg` is "RS256" (NOT the default!)
    unimplemented!();

    // TODO: VERIFY HEADER `kid` correspond to one of the public keys listed at https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com
    unimplemented!();

    // TODO: Verify ID token payload fields
    // ... ... ... ... ... ... ... ... ...

    // TODO: aud must be your Firebase project ID, the unique identifier for your Firebase project, which can be found in the URL of that project's console.
    unimplemented!();

    // TODO: iss must be "https://securetoken.google.com/<projectId>", where <projectId> is the same project ID used for aud above.
    unimplemented!();

    // TODO: exp, iat auto-validated by the lib
    unimplemented!()
}

/// Describes whether authorization was successfuly, and if it wasn't, describes why it failed.
enum AuthResult {
    Valid,
    Invalid(AuthFailureReason),
}

/// Why authorization failed:
///
/// - `Basic`: validation by the underlying `jsonwebtoken` crate failed somehow (exp cliam, iat cliam, alg header)
/// - `SubjectClaim`: this user isn't allowed
/// - `KidHeader`: key ID header doesn't correspond to a public key listed by google
/// - `IssuerClaim`: project ID in `iss` doesn't match expected project ID
/// - `AudienceClaim`: doesn't correspond to expected project ID
/// - `AuthTimeClaim`: claim value is not in the past
/// - `UnverifiedSignature`: ID token was not signed by the private key corresponding to the token's kid claim. Grab the public key from https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com and use the `jsonwebtoken` library to verify the signature. Use the value of max-age in the Cache-Control header of the response from that endpoint to know when to refresh the public keys.
enum AuthFailureReason {
    Basic,
    SubjectClaim,
    KidHeader,
    IssuerClaim,
    AudienceClaim,
    AuthTimeClaim,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::time::SystemTime;

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

    // TODO TODO TODO TODO
    #[test]
    fn encode_claims() {
        let sub = SubjectClaim("abc".to_string());
        let claims = FirebaseClaims {
            sub,
            aud: "nope".to_string(),   // TODO
            iss: "foogle".to_string(), // TODO
            exp: later(3600),
            iat: earlier(3600),
            auth_time: earlier(3600),
        };

        // TODO this header isn't reasonable
        let encoded = jwt::encode(&jwt::Header::default(), &claims, "secret".as_ref()).unwrap();

        assert!(!encoded.is_empty())
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
    fn validate_subject_claim() {
        let expect_valid =
            SubjectClaim("whoever".to_string()).validate(vec!["someone", "nobody", "whoever"]);
        assert!(expect_valid)
    }
}
