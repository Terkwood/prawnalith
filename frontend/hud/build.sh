#!/bin/sh

set -ex

wasm-pack build
cp pkg/*.js .
cp pkg/*.ts .
cp pkg/*.wasm .
cp index.html pkg/.
cp index.js pkg/.
cp package.json pkg/. # pre-printed for npm serve command to work
cp webpack.config.js pkg/.
cd pkg
npm install
npm run serve
