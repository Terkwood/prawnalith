#!/bin/bash

# see https://ownyourbits.com/2018/06/27/running-and-building-arm-docker-containers-in-x86/
cp /usr/bin/qemu-arm-static .

docker build -f Dockerfile.test -t sensor-tracker-armhf:testing
