#!/bin/bash

bash ./wait-for-it.sh 0.0.0.0:1883 -- ./run_led_status.sh & 
bash ./wait-for-it.sh 0.0.0.0:1883 -- ./run_sensor_tracker.sh & 
bash ./wait-for-it.sh 0.0.0.0:36379 -- ./run_ph_ref.sh &
