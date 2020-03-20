#!/bin/bash
PROTO=${STREAM_PROTO:="udp"}

gst-launch-1.0 ${PROTO}src port=5001 ! application/x-rtp,encoding-name=JPEG,payload=26 ! rtpjpegdepay ! jpegdec ! videoconvert ! autovideosink
