#!/bin/bash
# allow rocket.rs to serve traffic on port 80
# run this within the raspberry pi host
sudo iptables -A INPUT -p tcp --dport 80 -j ACCEPT
