#!/bin/bash
if [ -z "$1" ]
  then
    FILENAME=lo_fi
fi

rm /tmp/$FILENAME.h264
rm /tmp/$FILENAME.mp4
raspivid -t 30000 -w 640 -h 480 -fps 25 -b 1200000 -p 0,0,640,480 -o /tmp/$FILENAME.h264 && MP4Box -add /tmp/$FILENAME.h264 /tmp/$FILENAME.mp4
