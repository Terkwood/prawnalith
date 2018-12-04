#!/bin/bash
if [ -z "$1" ]
  then
    FILENAME=hi_fi
fi

rm /tmp/$FILENAME.h264
rm /tmp/$FILENAME.mp4
raspivid -t 12364 -o /tmp/$FILENAME.h264 && MP4Box -add /tmp/$FILENAME.h264 /tmp/$FILENAME.mp4
