#!/bin/bash

mkdir -p influxdb_volume
mkdir -p grafana_volume
mkdir -p telegraf_volume
mkdir -p redis_volume
docker build base/. -t prawnalith/local/base
docker-compose build
