use crate::authentication::{authenticate, AuthenticationResult};
use crate::authorization::authorize;
use crate::config::Config;
use crate::key_pairs;
use crate::redis_conn::*;
use crate::tanks;
use rocket::http::hyper::header::{AccessControlAllowOrigin, AccessControlMaxAge};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use rocket_contrib::json::Json;

/// This route requires that you authenticate using
/// a Firebase-signed JWT.
/// If redis blows up, the error will be logged using Debug,
/// and an opaque 500 status message will be returned to the caller.
/// This route will respond with the application origin whitelisted
/// using our `Config` struct's `cors_allowed_origin` property.
#[get("/tanks")]
pub fn tanks(
    _user: AuthorizedUser,
    conn: RedisDbConn,
    config: State<Config>,
) -> Result<CorsResponder, redis::RedisError> {
    Ok(CorsResponder {
        inner: Json(tanks::fetch_all(conn, &config.redis_namespace)?),
        header: config
            .cors_allow_origin
            .clone()
            .map(|allow_origin| AccessControlAllowOrigin::Value(allow_origin))
            .unwrap_or(AccessControlAllowOrigin::Any),
    })
}

#[derive(Responder)]
#[response(content_type = "json")]
pub struct CorsResponder {
    inner: Json<Vec<tanks::Tank>>,
    header: AccessControlAllowOrigin,
}

#[options("/tanks")]
pub fn tanks_options(config: State<Config>) -> PreflightOptionsResponder {
    PreflightOptionsResponder {
        inner: (),
        origin: config
            .cors_allow_origin
            .clone()
            .map(|allow_origin| AccessControlAllowOrigin::Value(allow_origin))
            .unwrap_or(AccessControlAllowOrigin::Any),
        methods: rocket::http::Header::new("AccessControlAllowMethods", "GET"),
        max_age: AccessControlMaxAge(86400),
    }
}

#[derive(Responder)]
pub struct PreflightOptionsResponder {
    inner: (),
    origin: AccessControlAllowOrigin,
    methods: rocket::http::Header<'static>,
    max_age: AccessControlMaxAge,
}

#[derive(Debug)]
pub struct AuthorizedUser {
    uid: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthorizedUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthorizedUser, ()> {
        let auth_headers: Vec<_> = request.headers().get("Authorization").collect();
        if auth_headers.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let bearer_string = auth_headers.get(0);
        if let None = bearer_string {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        let token = token_from_bearer_string(bearer_string.unwrap());
        if let Err(_) = token {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let redis_namespace: String = request
            .guard::<State<Config>>()?
            .inner()
            .redis_namespace
            .to_string();
        let redis_conn = request.guard::<RedisDbConn>()?;
        let config: &Config = request.guard::<State<Config>>()?.inner();

        match key_pairs::fetch_from_redis(&redis_conn, &redis_namespace) {
            Err(_) => Outcome::Failure((Status::InternalServerError, ())),
            Ok(key_pairs) => {
                match authenticate(&token.unwrap(), key_pairs, &config.firebase_project_id) {
                    AuthenticationResult::Invalid(_) => {
                        Outcome::Failure((Status::Unauthorized, ()))
                    }
                    AuthenticationResult::Valid(uid) => match authorize(
                        uid.clone(),
                        &RedisConnContext {
                            namespace: redis_namespace,
                            conn: redis_conn,
                        },
                    ) {
                        Ok(true) => Outcome::Success(AuthorizedUser { uid: uid.0 }),
                        _ => Outcome::Failure((Status::Unauthorized, ())),
                    },
                }
            }
        }
    }
}

fn token_from_bearer_string(bearer_string: &str) -> Result<String, ()> {
    let v: Vec<&str> = bearer_string.split(' ').collect();
    if let Some(token) = v.get(1) {
        // fails #1: invalid sig
        Ok(token.to_string())
    } else {
        Err(())
    }
}

pub fn startup(config: Config) {
    rocket::ignite()
        .manage(config)
        .attach(RedisDbConn::fairing())
        .mount("/", routes![tanks, tanks_options])
        .launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn auth_header_split_ok() {
        let good = "Bearer OfJollity";
        let actual = token_from_bearer_string(good);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), "OfJollity")
    }

    #[test]
    fn auth_header_split_fail() {
        let bad = "BeerPlease";
        let actual = token_from_bearer_string(bad);
        assert!(actual.is_err())
    }
}
