#!/bin/bash
raspivid -o /tmp/loop.h264 && MP4Box -add /tmp/loop.h264 /tmp/loop.mp4
