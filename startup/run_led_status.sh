#!/bin/bash

export $(grep -v '^#' .env | xargs)
cd ../services/led_status_helper
$LED_BIN
