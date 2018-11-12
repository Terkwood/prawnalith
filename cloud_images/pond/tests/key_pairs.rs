use pond::key_pairs::*;

use std::collections::HashMap;

#[test]
fn cert_to_pub_key() {
    let cert = PubCert(
        r#"-----BEGIN CERTIFICATE-----
MIIDHDCCAgSgAwIBAgIIGLZwido7rJUwDQYJKoZIhvcNAQEFBQAwMTEvMC0GA1UE
AxMmc2VjdXJldG9rZW4uc3lzdGVtLmdzZXJ2aWNlYWNjb3VudC5jb20wHhcNMTgx
MTA5MjEyMDQzWhcNMTgxMTI2MDkzNTQzWjAxMS8wLQYDVQQDEyZzZWN1cmV0b2tl
bi5zeXN0ZW0uZ3NlcnZpY2VhY2NvdW50LmNvbTCCASIwDQYJKoZIhvcNAQEBBQAD
ggEPADCCAQoCggEBALhTHfVBvUaD1TXq9rtfaEX/bDIdYHtzFKsMEit8XOMRxpKC
C1w4M3OqWOQFJezJr8PfirwBDXeR15kS1rRMurLhRhU/kGAoQy4/gYUoz9PssVvI
7xafvR9gB4+HGSVc5bc51/+sBS1qoIv5gixruJSWdpjtoN89UJ/OvLXGZAgukdip
DgG4fMu+GxycCxFzlBFI8yoANwbbJuxQMejCDrpVnUfsR9oZ/rsNqRTbkz/dgrQJ
QGkfsmlvsiuKeflDKaw9MGvUjOmZZB5sbdrbygkGG8CFH+TmXw456DnW1OhCSaci
39+r2407M/0Ze8j9bu0RH4fJKvtfStVKg68zzWkCAwEAAaM4MDYwDAYDVR0TAQH/
BAIwADAOBgNVHQ8BAf8EBAMCB4AwFgYDVR0lAQH/BAwwCgYIKwYBBQUHAwIwDQYJ
KoZIhvcNAQEFBQADggEBAA1PqG5IUb8jfftyvL6sGrHI7IzH3w3fncv/T/pAOTyt
71JCDI9LBseZ9vFeZqtxX4LalbT4aZW2/wTtdPHpb5TxtWX3gW9umu0xM3n0WSjW
PqPOXLvmLYgmCt3ZVrxhhQ+O/ouxO8RxLmktjGPunCoB/DHARfmj+t5jVTR1Jmv5
cQkuiby96FkEIB8sUlVXu6it91PxeJOlecFSUBBJiI+CcLfkOLgDSkVJvoN9W0dq
XA2bqiND10dZk8X412YRViRLbXjpPboH0eCAQBJR1ZT3r2BBBMtYlw2FLd+Tn0LT
GBuGIURzpq5Ukj1he/bNxcimcO6sDE7WRqacbEFrbUo=
-----END CERTIFICATE-----
"#
        .to_string(),
    );
    let actual = cert.to_pub_key();

    assert!(actual.is_ok());
    let expected = PubKey(
        r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAuFMd9UG9RoPVNer2u19o
Rf9sMh1ge3MUqwwSK3xc4xHGkoILXDgzc6pY5AUl7Mmvw9+KvAENd5HXmRLWtEy6
suFGFT+QYChDLj+BhSjP0+yxW8jvFp+9H2AHj4cZJVzltznX/6wFLWqgi/mCLGu4
lJZ2mO2g3z1Qn868tcZkCC6R2KkOAbh8y74bHJwLEXOUEUjzKgA3Btsm7FAx6MIO
ulWdR+xH2hn+uw2pFNuTP92CtAlAaR+yaW+yK4p5+UMprD0wa9SM6ZlkHmxt2tvK
CQYbwIUf5OZfDjnoOdbU6EJJpyLf36vbjTsz/Rl7yP1u7REfh8kq+19K1UqDrzPN
aQIDAQAB
-----END PUBLIC KEY-----
"#
        .to_string(),
    );
    assert_eq!(actual.unwrap(), expected)
}

#[test]
fn deserialize_pub_key_json() {
    let json = r#"{
  "deadbeef": "-----BEGIN CERTIFICATE-----\nthistles=\n-----END CERTIFICATE-----\n",
  "f00d1111": "-----BEGIN CERTIFICATE-----\nclap=\n-----END CERTIFICATE-----\n"
}
"#;
    let deser: Result<HashMap<SigningKeyId, PubCert>, _> = serde_json::from_str(json);
    assert!(deser.is_ok());
    let hash = deser.unwrap();
    assert_eq!(
        hash.get(&SigningKeyId("deadbeef".to_string())).unwrap(),
        &PubCert("-----BEGIN CERTIFICATE-----\nthistles=\n-----END CERTIFICATE-----\n".to_string())
    )
}
