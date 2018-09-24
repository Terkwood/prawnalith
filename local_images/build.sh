#!/bin/bash

docker build base/. -t prawnalith/local/base
docker-compose build
