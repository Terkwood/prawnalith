use super::data::Key;

/// Represents a change to a value in Redis.
/// Currently only supports the minimum combinations
/// of key/value used by the prawnalith.
///

enum RDelta<'a, 'b, 'c, T> {
    #[serde(borrow)]
    AddSetMember(Key<'a, 'b>, RVal<'c, T>),
    UpdateHashField(Key<'a, 'b>, RVal<'c, T>),
    UpdateKey(Key<'a, 'b>, RVal<'c, T>),
}

/// A value which is stored in Redis.
struct RVal<'a, T>(pub &'a T);
