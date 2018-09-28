#!/bin/bash

mkdir -p influxdb_volume
docker build base/. -t prawnalith/local/base
docker-compose build
