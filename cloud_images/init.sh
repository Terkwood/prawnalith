#!/bin/bash

sudo mkdir -p /var/volumes
sudo chown -R $USER:$USER /var/volumes
sudo chown -R $USER:$USER /var/prawnalith

cd /var/prawnalith

[[ -d /var/prawnalith ]] || git clone https://github.com/Terkwood/prawnalith 

echo alias docker-compose="'"'docker run --rm \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -v "$PWD:/rootfs/$PWD" \
    -w="/rootfs/$PWD" \
    docker/compose:1.13.0'"'" >> ~/.bashrc
echo alias dc="'"'docker-compose'"'" >> ~/.bashrc

echo alias htop="'"'docker run --rm -it --pid host frapsoft/htop'"'" >> ~/.bashrc
echo alias gfp="'"'git fetch && git pull'"'" >> ~/.bashrc
echo alias gc="'"'git checkout'"'" >> ~/.bashrc
echo alias cdw="'"'cd /var/prawnalith/src'"'" >> ~/.bashrc

docker pull rust
docker pull frapsoft/htop

# install systemd scripts
sudo cp /var/prawnalith/systemd/*.service /etc/systemd/system/.
for i in *.service; do [ -f "$i" ] && sudo systemctl enable $i && sudo systemctl start $i; done
