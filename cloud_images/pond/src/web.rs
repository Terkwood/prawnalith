use crate::authentication::{authenticate, AuthenticationResult};
use crate::authorization::authorize;
use crate::config::Config;
use crate::key_pairs;
use crate::push::{PushData, PushDataError};
use crate::redis_conn::*;
use crate::tanks;
use rocket::http::hyper::header::{AccessControlAllowOrigin, AccessControlMaxAge};
use rocket::http::RawStr;
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

const ONE_DAY: u32 = 86400;

#[options("/tanks")]
pub fn tanks_options(config: State<Config>) -> PreflightOptionsResponder {
    PreflightOptionsResponder {
        inner: (),
        allow_origin: config
            .cors_allow_origin
            .clone()
            .map(|allow_origin| AccessControlAllowOrigin::Value(allow_origin))
            .unwrap_or(AccessControlAllowOrigin::Any),
        allow_methods: rocket::http::Header::new("Access-Control-Allow-Methods", "GET"),
        allow_headers: rocket::http::Header::new("Access-Control-Allow-Headers", "Authorization"),
        max_age: AccessControlMaxAge(ONE_DAY),
    }
}

#[derive(Responder)]
pub struct PreflightOptionsResponder {
    inner: (),
    allow_origin: AccessControlAllowOrigin,
    allow_methods: rocket::http::Header<'static>,
    allow_headers: rocket::http::Header<'static>,
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

/// An endpoint which receives push messages from Google pub/sub platform.
/// These messages summarize changes to the Redis database hosted in
/// in local proximity to the temp & ph sensors.
/// See https://cloud.google.com/pubsub/docs/push
///
/// Here is an example of sending a base64 encoded payload to the endpoint.
///
/// ```sh
/// curl -k -d '{ "message": { "attributes": { "key": "value"  }, "data": "eyJ1cGRhdGVfaGFzaCI6eyJrZXkiOiJwcmF3bmJhYnkvc2Vuc29ycy90ZW1wL2FhYWFhYWFhLWVlZWUtYWFhYS1hYWFhLWFhYWFhYWFhYWFhYSIsImZpZWxkcyI6W3sibmFtZSI6InRlbXBfdXBkYXRlX2NvdW50IiwidmFsIjoiNDEwOTY2In0seyJuYW1lIjoidGVtcF91cGRhdGVfdGltZSIsInZhbCI6IjE1NDI3NTI3MTAifSx7Im5hbWUiOiJ0ZW1wX2MiLCJ2YWwiOiIyNC42MiJ9LHsibmFtZSI6InRlbXBfZiIsInZhbCI6Ijc2LjMyIn1dLCJ0aW1lIjoxNTQyNzUyNzE1fX0=", "message_id": "136969346945" },"subscription": "projects/myproject/subscriptions/mysubscription"}' -H "Content-Type: application/json" -X POST https://localhost:8000/push_redis\?token\=fancy_shared_sekrit
/// ```
///
/// In this case, the base64 "data" attribute decodes as follows:
/// ```json
/// {"update_hash":{"key":"prawnbaby/sensors/temp/aaaaaaaa-eeee-aaaa-aaaa-aaaaaaaaaaaa","fields":[{"name":"temp_update_count","val":"410966"},{"name":"temp_update_time","val":"1542752710"},{"name":"temp_c","val":"24.62"},{"name":"temp_f","val":"76.32"}],"time":1542752715}}
/// ```
#[post(
    "/push_redis?<token>",
    format = "application/json",
    data = "<data>"
)]
pub fn push_redis(
    data: Json<PushData>,
    token: &RawStr,
    conn: RedisDbConn,
    config: State<Config>,
) -> Status {
    let push_secret: String = config.push_secret.to_string();
    // This can be improved.
    // See https://github.com/Terkwood/prawnalith/issues/60
    if token.as_str() == push_secret {
        match data.ingest(conn) {
            Ok(_) => Status::NoContent,
            Err(PushDataError::Redis) => Status::InternalServerError,
            Err(_) => Status::UnprocessableEntity,
        }
    } else {
        Status::Unauthorized
    }
}

pub fn startup(config: Config) {
    rocket::ignite()
        .manage(config)
        .attach(RedisDbConn::fairing())
        .mount("/", routes![tanks, tanks_options, push_redis])
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
