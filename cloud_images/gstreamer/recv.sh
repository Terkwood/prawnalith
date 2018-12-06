#!/bin/bash

gst-launch-1.0 tcpserversrc host=0.0.0.0 port=5001 \
	! h264parse !  x264enc ! mpegtsmux ! hlssink
