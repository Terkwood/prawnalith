#!/bin/bash

gst-launch-1.0 tcpserversrc host=0.0.0.0 port=5001 \
	! rtph264depay ! h264parse ! avdec_h264 \
	! x264enc ! mpegtsmux ! hlssink
