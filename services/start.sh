#!/bin/bash

./run_led_status.sh & 
./run_sensor_tracker.sh & 
./run_ph_ref.sh &
# HACKY :-D
cd ~/temp_sensor_tracker_for_test_test
~/sensor_tracker
