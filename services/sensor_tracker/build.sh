#!/bin/bash

# see https://ownyourbits.com/2018/06/27/running-and-building-arm-docker-containers-in-x86/
# see https://matchboxdorry.gitbooks.io/matchboxblog/content/blogs/build_and_run_arm_images.html
cp /usr/bin/qemu-arm-static .

docker build . -f Dockerfile.test -t prawnalith/sensor_tracker:test
docker build . -f Dockerfile.prod -t prawnalith/sensor_tracker:prod