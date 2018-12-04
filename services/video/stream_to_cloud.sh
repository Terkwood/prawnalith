#!/bin/bash

TARGET_IP=localhost
PORT=8888

# note that you can vary the framerate and width/height to easy
# success.

gst-launch-1.0 -v rpicamsrc  ! videoconvert ! videoscale ! \
        video/x-raw,format=I420,width=320,height=240,framerate=15/1 ! \
        jpegenc ! rtpjpegpay ! gdppay ! tcpclientsink host=$TARGET_IP port=$PORT
