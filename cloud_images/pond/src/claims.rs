use crate::authentication::{AuthenticationFailure, AuthenticationResult};
use std::time::SystemTime;
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FirebaseClaims {
    pub sub: SubjectClaim,
    pub iss: String,
    pub aud: String,
    pub iat: usize,
    pub exp: usize,
    pub auth_time: usize,
}
impl FirebaseClaims {
    /// Validate all firebase claims which aren't related to the signature or
    /// signing algorithm:  exp, iss, aud, auth_time, sub (non-empty), etc.
    pub fn validate(&self, project_id: &str) -> AuthenticationResult {
        let just_now = epoch_secs() as usize;
        match self {
            claims if claims.sub.0.is_empty() => {
                AuthenticationResult::Invalid(AuthenticationFailure::MissingSubject)
            }

            claims if claims.exp <= just_now => {
                AuthenticationResult::Invalid(AuthenticationFailure::Expired)
            }

            claims if claims.auth_time >= just_now => {
                AuthenticationResult::Invalid(AuthenticationFailure::AuthTime)
            }

            claims if claims.aud != project_id => {
                AuthenticationResult::Invalid(AuthenticationFailure::Audience)
            }

            claims if claims.iss != format!("https://securetoken.google.com/{}", project_id) => {
                AuthenticationResult::Invalid(AuthenticationFailure::Issuer)
            }

            claims if claims.iat >= just_now => {
                AuthenticationResult::Invalid(AuthenticationFailure::IssuedAt)
            }

            claims => AuthenticationResult::Valid(claims.sub.clone()),
        }
    }
}

/// Must be a non-empty string and must be the uid of the user or device
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SubjectClaim(pub String);
impl SubjectClaim {
    pub fn authenticate(&self) -> bool {
        !self.0.is_empty()
    }
}

fn epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
