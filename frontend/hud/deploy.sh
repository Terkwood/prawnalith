#!/bin/bash

cargo web build --release
cp target/asmjs-unknown-emscripten/release/hud.js static/.
firebase deploy
