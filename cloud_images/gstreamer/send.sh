#!/bin/bash

# Use this script on a raspberry pi to
# send data to a cloud instance.
# You can use localhost as your target IP
# if your cloud instance is 
# protected by an SSH tunnel.

TARGET_IP=localhost
PORT=8888

# note that you can vary the framerate and width/height to easy
# success.

gst-launch-1.0 -v rpicamsrc  ! videoconvert ! videoscale \
        ! video/x-raw,format=I420,width=320,height=240,framerate=15/1 \
        ! x264enc tune="zerolatency" threads=1 ! h264parse ! rtph264pay \
	! tcpclientsink host=$TARGET_IP port=$PORT
