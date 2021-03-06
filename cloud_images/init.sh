#!/bin/bash

[[ -d /var/prawnalith ]] || git clone https://github.com/Terkwood/prawnalith 

sudo mv prawnalith /var/.
sudo chown -R $USER:$USER /var/prawnalith

sudo mkdir -p /var/volumes
sudo chown -R $USER:$USER /var/volumes

cd /var/prawnalith

echo alias docker-compose="'"'docker run --rm \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -v "$PWD:/rootfs/$PWD" \
    -w="/rootfs/$PWD" \
    docker/compose:1.13.0'"'" >> ~/.bashrc
echo alias dc="'"'docker-compose'"'" >> ~/.bashrc
echo alias de="'"'docker exec'"'" >> ~/.bashrc
echo alias gfp="'"'git fetch && git pull'"'" >> ~/.bashrc
echo alias gc="'"'git checkout'"'" >> ~/.bashrc
echo alias cdw="'"'cd /var/prawnalith'"'" >> ~/.bashrc

# install systemd scripts
sudo cp /var/prawnalith/cloud_images/systemd/*.service /etc/systemd/system/.
cd /var/prawnalith/cloud_images/systemd
for i in *.service; do [ -f "$i" ] && sudo systemctl enable $i && sudo systemctl start $i; done
