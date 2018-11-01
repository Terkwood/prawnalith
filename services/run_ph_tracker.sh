#!/bin/bash

export $(grep -v '^#' .env | xargs)
cd ../services/ph_tracker
$PH_TRACKER_BIN
