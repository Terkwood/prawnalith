#!/bin/bash

echo alias docker-compose="'"'docker run --rm \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -v "$PWD:/rootfs/$PWD" \
    -w="/rootfs/$PWD" \
    docker/compose:1.13.0'"'" >> ~/.bashrc

echo alias htop="'"'docker run --rm -it --pid host frapsoft/htop'"'" >> ~/.bashrc
echo alias gfp="'"'git fetch && git pull'"'" >> ~/.bashrc
echo alias gc="'"'git checkout'"'" >> ~/.bashrc
echo alias cdw="'"'cd ~/prawnalith'"'" >> ~/.bashrc

docker pull rust
docker pull frapsoft/htop
