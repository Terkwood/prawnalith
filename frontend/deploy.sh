#!/bin/bash

cargo web build --target=wasm32-unknown-unknown --release &&
  cp target/wasm32-unknown-unknown/release/*.js static/. &&
  cp target/wasm32-unknown-unknown/release/*.wasm static/. &&
  firebase deploy
