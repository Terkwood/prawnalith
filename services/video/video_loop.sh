#!/bin/bash
rm /tmp/loop.h264
rm /tmp/loop.mp4
raspivid -o /tmp/loop.h264 -t 12364 && MP4Box -add /tmp/loop.h264 /tmp/loop.mp4
