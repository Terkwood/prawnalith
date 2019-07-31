use crate::claims::SubjectClaim;
use crate::redis_conn::RedisConnContext;
use crate::rocket_contrib::databases::redis::Commands;

/// Authorizes a user based on whether they are allowed to access
/// the system.  We track a redis SET of firebase UIDs in order
/// to keep track of our authorized users.
pub fn authorize(
    firebase_uid: SubjectClaim,
    redis_context: &RedisConnContext,
) -> Result<bool, rocket_contrib::databases::redis::RedisError> {
    let frag = "pond/firebase/authorized_uids";
    let key = format!("{}/{}", redis_context.namespace, frag);
    Ok(redis_context.conn.sismember(key, firebase_uid.0)?)
}
