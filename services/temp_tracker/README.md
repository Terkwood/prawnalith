# Sensor tracker utility

Listens for various sensor reports.  Creates an entry in the
Redis sensors set if a reporting sensor doesn't exist.  Creates
a stubbed hash which the user can fill out in order to associate
tank, manufacturing data, etc.

## On temp sensor report:

This is useful for sensors generating temperature data.

Such data comes into an MQTT topic looking like this:

```
{ "device_id": <hex>, "temp_f", "temp_c" }
```

This utility first checks to see whether the temp sensor is 
already known to us.

```
SISMEMBER <namespace>:temp_sensors
```

If it isn't, it will create the following type of stub record
for the temp sensor based on a UUID V5 ID conversion:

```
HMSET <namespace>:temp_sensors/<uuid_v5_id> start_date <epoch>
```

The operator is encouraged to later amend the hash to include
a helpful reference to the tank which the sensor serves, so
that the LED status utility can properly format messages.

```
HSET <namespace>:temp_sensors/<hex_id> tank 0
```
