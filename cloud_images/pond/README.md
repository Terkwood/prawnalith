# Pond Service

[![Docker badge](https://img.shields.io/docker/pulls/prawnalith/pond.svg)](https://hub.docker.com/r/prawnalith/pond/)

A small webservice used to serve aquarium data (temp and pH)
over the  🌎 World 🦀 Wide 🦐 Web 🕸.

## Authorization via Firebase

We follow Firebase reccomendations to validate Json Web Tokens (JWTs)
provided to the web service.  See https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library for more information.
