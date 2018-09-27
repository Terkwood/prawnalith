# Collection of docker images for local Raspberry Pi

We have several utilities exposed for use in a local network:

- influxdb
- telegraf
- redis
- mosquitto

You can use [build.sh](build.sh) to build these images. You can use `docker-compose` to start these images.  

The idea is to have ESP8266 and other Pis pushing data to an MQTT broker which is built in the [Dockerfile for mosquitto](mosquitto/Dockerfile).  This data is then processed by `telegraf` and pushed into an `InfluxDB` instance, and also pushed into the cloud via MQTT.

We plan to write a [very minimal HTTP service](rocket/) which will present data that can be consumed by the frontend.

## Telegraf Configuration

We use [Telegraf](https://www.influxdata.com/time-series-platform/telegraf/) to transfer sensor data from the local MQTT broker into InfluxDB, to mirror basic sensor data from the local MQTT broker into the cloud MQTT broker, and to push messages about temp, pH levels, etc to local LED screens.

Telegraf is driven by a configuration file _which must be present in the same directory as docker-compose.yml_.

You can find an [example of a sample configuration here](sample_telegraf.conf).

To start up your own instance, try copying the sample config, then editing it yourself:

```
cp sample_telegraf.conf telegraf.conf
vi telegraf.conf
```

### Sample Influx Queries

Basic stuff, but...

```
influx -database 'prawnalith' -execute 'select * from mqtt_consumer order by time DESC LIMIT 10'
```

## Resources

- Useful description of how to tie Influx & Telegraf together, on ARM, using only docker containers: https://community.influxdata.com/t/influxdata-docker-on-arm/2493
