#!/bin/bash

export $(grep -v '^#' .env | xargs)
cd services/sensor_tracker
$SENSOR_TRACKER_BIN
