#!/bin/bash

# Your grafana volume wants to be owned by the standard
# grafana user.  This script sets it up in the host.
# 
#

sudo adduser --uid 472 --gid 472 grafana

echo "You should _also add an entry_ in `/etc/group` for"
echo "grafana, on your host machine:"

echo ""
echo "    grafana:x:472:pi"
echo ""
