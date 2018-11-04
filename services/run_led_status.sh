#!/bin/bash

export $(grep -v '^#' .env | xargs)
cd led_status_helper
$LED_BIN
