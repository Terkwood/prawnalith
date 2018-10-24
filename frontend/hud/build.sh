#!/bin/sh

set -ex

wasm-pack build
cp pkg/*.js .
cp pkg/*.ts .
cp pkg/*.wasm .
npm install
npm run serve
