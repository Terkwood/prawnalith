#!/bin/bash

gst-launch-1.0 tcpserversrc host=0.0.0.0 port=5001 \
	! rtph264depay ! h264parse ! mpegtsmux ! hlssink
