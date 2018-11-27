use pond::push::*;
use std::collections::HashMap;

#[test]
fn test_message_verification() {
    let shared_secret = "sekrit".as_bytes();
    let expected_sig = "M7HSodfA0G0vHcvaoAsdoFCZk9hj0Dqo9JFX6C1YXjI=";
    let message_data = "AA==";

    let mut attrs = HashMap::new();
    attrs.insert("sig".to_owned(), expected_sig.to_owned());
    let message = Message {
        attributes: Some(attrs),
        data: Base64(message_data.to_owned()),
        message_id: "0".to_owned(),
    };

    assert!(message.verify_signature(&shared_secret))
}
