use super::data::Key;

/// Represents a change to a value in Redis.
/// Currently only supports the minimum combinations
/// of key/value used by the prawnalith.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RDelta<'a, 'b> {
    AddSetMember {
        #[serde(borrow)]
        key: &'a str,
        #[serde(borrow)]
        rval: RVal<'b>,
    },
    UpdateHashField {
        key: &'a str,
        rval: RVal<'b>,
    },
    UpdateKey {
        key: &'a str,
        rval: RVal<'b>,
    },
}

/// A value which is stored in Redis.
/// It's always represented as a string so that
/// we can easily cope with deserialization & serialization.
#[derive(Serialize, Deserialize)]
struct RVal<'a>(pub &'a str);

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::{Key, Namespace};
    use serde_json;

    fn ns() -> Namespace<'static> {
        Namespace("prawnspace")
    }

    #[test]
    fn rval_ser() {
        let counter = &RVal("1");
        assert_eq!(serde_json::to_string(counter).unwrap(), "\"1\"");
    }

    #[test]
    fn add_set_member_ser() {
        let set_friend = &RDelta::AddSetMember {
            key: &Key::AllSensorTypes { ns: ns() }.to_string(),
            rval: RVal("foo"),
        };
        println!("{}", serde_json::to_string(set_friend).unwrap());
    }

}
