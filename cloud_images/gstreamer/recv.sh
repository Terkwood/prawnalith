#!/bin/bash

gst-launch-1.0 tcpserversrc host=0.0.0.0 port=5001 \
	! application/x-rtp,clock-rate=90000,payload=96 \
	! rtph264depay ! mpegtsmux ! hlssink
