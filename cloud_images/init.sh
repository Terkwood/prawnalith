#!/bin/bash

echo alias docker-compose="'"'docker run --rm \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -v "$PWD:/rootfs/$PWD" \
    -w="/rootfs/$PWD" \
    docker/compose:1.13.0'"'" >> ~/.bashrc

docker pull frapsoft/htop
echo alias htop="docker run --rm -it --pid host frapsoft/htop"
