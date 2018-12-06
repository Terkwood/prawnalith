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

gst-launch-1.0 -v rpicamsrc \
	! queue ! videoconvert ! queue ! videoscale \
	! queue ! video/x-raw,format=I420,width=320,height=240,framerate=15/1 \
	! queue ! x264enc tune="zerolatency" threads=1 ! queue \
	! video/x-h264,stream-format=byte-stream \
	! tcpclientsink host=$TARGET_IP port=$PORT
