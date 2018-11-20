# Pond Service

[![Docker badge](https://img.shields.io/docker/pulls/prawnalith/pond.svg)](https://hub.docker.com/r/prawnalith/pond/)

A small webservice used to serve aquarium data (temp and pH)
over the 🌎 World 🦀 Wide 🦐 Web 🕸.

It is capable of authenticating and authorizing OAuth2-compliant Json Web Tokens
(JWT)s provided by Google Firebase.  It requires that a list of authorized
user IDs be kept in a Redis database.

## Example

```
curl -k -H "Authorization: Bearer $FIREBASE_JWT" https://$FIREBASE_HOST/tanks | python -m json.tool
```  

```
[
    {
        "id": 1,
        "name": "The Mothership",
        "ph": 7.72,
        "ph_mv": 461.05,
        "ph_update_count": 2454127,
        "ph_update_time": 1542744006,
        "temp_c": 27.56,
        "temp_f": 81.61,
        "temp_update_count": 2535781,
        "temp_update_time": 1542744006
    },
    {
        "id": 2,
        "name": "The Pond",
        "temp_c": 24.62,
        "temp_f": 76.32,
        "temp_update_count": 191511,
        "temp_update_time": 1542744003
    }
]
```

## Authorization via Firebase

We follow Firebase reccomendations to validate Json Web Tokens (JWTs)
provided to the web service.  See https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library for more information.

Further, this project provides `sub` (subject) claim validation specific to the prawnalith: the firebase UID provided in the subject claim must be a member of a list of authorized users.

## Redis data scheme

Authorized firebase UIDs are stored as a Redis SET at the key `{namespace}/pond/firebase/authorized_uids`

Google public RSA signing keys are stored as a Redis HASH at the key `{namespace}/pond/firebase/public_signing_keys`

### Test with cURL

```sh
FIREBASE_JWT=ey... curl -k -H "Authorization: Bearer $FIREBASE_JWT" https://localhost:8000/tanks
```

### Tiny docker image

The docker image created as a result of this effort uses the libmusl builder image from https://github.com/emk/rust-musl-builder. As a result, the alpine-based image is small -- around 10MB.

The image for this program is [available on docker hub](https://hub.docker.com/r/prawnalith/pond/).

## Thank You

- Thanks to Alex Maslakov and contributors to [frank_jwt library for rust](https://github.com/GildedHonour/frank_jwt).
- We really appreciate the effort from Eric Kidd, who created [rust musl builder for Alpine linux / docker](https://github.com/emk/rust-musl-builder).  Thank you.
- Thanks to [SwiftyRSA](https://github.com/TakeScoop/SwiftyRSA) for the public & private key examples used in our tests.
- `redis` & `redis-rs` should always be given acclaim.  Thank you!
