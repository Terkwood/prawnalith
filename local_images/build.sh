#!/bin/bash

#mkdir -p grafana_volume
mkdir -p redis_volume
docker build base/. -t prawnalith/local/base
docker build rust/. -t prawnalith/local/rust
docker-compose build
