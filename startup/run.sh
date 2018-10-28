#!/bin/bash


./run_led_status.sh &
P1=$!
./run_temp_tracker.sh & 
P2=$!
./run_ph_tracker.sh & 
P3=$!
./run_ph_ref.sh &
P4=$!

wait $P1 $P2 $P3 $P4

