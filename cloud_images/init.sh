#!/bin/bash

# see https://docs.docker.com/install/linux/docker-ce/debian/
apt-get remove docker docker-engine docker.io
apt-get update
apt-get install -y \
     apt-transport-https \
     ca-certificates \
     curl \
     gnupg2 \
     software-properties-common
# curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -

