# Pond Service

[![Docker badge](https://img.shields.io/docker/pulls/prawnalith/pond.svg)](https://hub.docker.com/r/prawnalith/pond/)

A small webservice used to serve aquarium data (temp and pH)
over the  🌎 World 🦀 Wide 🦐 Web 🕸.

It is capable of authenticating OAuth2-compliant Json Web Tokens
(JWT)s provided by Google Firebase.

## Authorization via Firebase

We follow Firebase reccomendations to validate Json Web Tokens (JWTs)
provided to the web service.  See https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library for more information.

Further, this project provides `sub` (subject) claim validation specific to the prawnalith: the firebase UID provided in the subject claim must be a member of a list of authorized users.

### Tiny docker image

The docker image created as a result of this effort uses the libmusl builder image from https://github.com/emk/rust-musl-builder. As a result, the alpine-based image is small -- around 10MB.

## Thank You

- We really appreciate the effort from Eric Kidd, who created [rust musl builder for Alpine linux / docker](https://github.com/emk/rust-musl-builder).  Thank you.
