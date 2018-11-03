use super::data::Key;

/// Represents a change to a value in Redis.
/// Currently only supports the minimum combinations
/// of key/value used by the prawnalith.
#[derive(Serialize, Deserialize)]
enum RDelta<'a, 'b, 'c> {
    AddSetMember {
        #[serde(borrow)]
        key: Key<'a, 'b>,
        #[serde(borrow)]
        rval: RVal<'c>,
    },
    UpdateHashField {
        key: Key<'a, 'b>,
        rval: RVal<'c>,
    },
    UpdateKey {
        key: Key<'a, 'b>,
        rval: RVal<'c>,
    },
}

/// A value which is stored in Redis.
/// It's always represented as a string so that
/// we can easily cope with deserialization & serialization.
#[derive(Serialize, Deserialize)]
struct RVal<'a>(pub &'a str);
