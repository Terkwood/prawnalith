# install systemd scripts
sudo cp /var/prawnalith/local_images/systemd/*.service /etc/systemd/system/.
cd /var/prawnalith/local_images/systemd
for i in *.service; do [ -f "$i" ] && sudo systemctl enable $i && sudo systemctl start $i; done
