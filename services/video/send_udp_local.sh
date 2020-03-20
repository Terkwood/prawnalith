#!/bin/bash

T=${TARGET_IP:="192.168.1.1"}
P=5001
echo "Sending to $T:$P"
gst-launch-1.0 -v rpicamsrc  ! videoconvert ! videoscale ! video/x-raw,format=I420,width=640,height=480,framerate=20/1 ! jpegenc ! rtpjpegpay ! udpsink host=${T} port=${P}
