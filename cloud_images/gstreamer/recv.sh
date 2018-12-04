#!/bin/bash

gst-launch-1.0 tcpserversrc host=0.0.0.0 port=5001 ! gdpdepay ! application/x-rtp,encoding-name=JPEG,payload=26 ! rtpjpegdepay ! jpegdec ! x264enc ! mpegtsmux ! hlssink
