#!/bin/bash

export $(grep -v '^#' .env | xargs)
cd ../services/temp_tracker
$TEMP_TRACKER_BIN
