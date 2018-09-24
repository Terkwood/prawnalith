# Collection of docker images for local Raspberry Pi

We have several utilities exposed for use in a local network:

- influxdb
- telegraf
- redis
- mosquitto

You can use [build.sh](build.sh) to build these images. You can use `docker-compose` to start these images.  

The idea is to have ESP8266 and other Pis pushing data to an MQTT broker which is built in the [Dockerfile for mosquitto](mosquitto/Dockerfile).  This data is then processed by `telegraf` and pushed into an `InfluxDB` instance, and also pushed into the cloud via MQTT.

We plan to write a [very minimal HTTP service](rocket/) which will present data that can be consumed by the frontend.

### Resources

- Useful description of how to tie Influx & Telegraf together, on ARM, using only docker containers: https://community.influxdata.com/t/influxdata-docker-on-arm/2493
