#!/bin/bash

sudo sysctl vm.overcommit_memory=1
sudo sh -c 'echo never > /sys/kernel/mm/transparent_hugepage/enabled'
/var/prawnalith/cloud_images/docker-compose.sh up

