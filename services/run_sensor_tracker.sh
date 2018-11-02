#!/bin/bash

export $(grep -v '^#' .env | xargs)
cd sensor_tracker
$SENSOR_TRACKER_BIN
