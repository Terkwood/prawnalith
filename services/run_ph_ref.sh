#!/bin/bash

export $(grep -v '^#' .env | xargs)
cd ph_ref_calibration
$PH_REF_BIN
