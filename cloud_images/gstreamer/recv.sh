#!/bin/bash

gst-launch-1.0 tcpserversrc host=0.0.0.0 port=5001 \
	! h264parse ! queue ! avdec_h264 ! queue ! x264enc \
	! queue ! mpegtsmux ! queue ! hlssink
