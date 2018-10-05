# LED status helper

Periodically generates a human-readable, full system report, which will
be shown on small LED matrix displays.

## Initial usage

The LED status helper checks all tanks for their current temperature
and pH values, and emits a report formatted as follows:

```
#1 82.30F pH 7.1 #2 83.11F pH 6.9
```

## Redis query pattern

We expect Redis to hold a counter which tells us how many tanks to query.

```
GET prawnalith/tanks
"2"
```

Each tank entry is checked for `temp_f`, `temp_c`, and `ph` fields
based on the most recent reading from its associated sensors.

```
HMGET prawnalith/tanks/1 temp_f temp_c ph
HMGET prawnalith/tanks/2 temp_f temp_c ph
```

A human needs to satisfy the link between a given temp sensor
and the tank it belongs to, e.g:

```
HSET prawnalith/temp_sensors/8e5e5899-161f-470e-bd79-e6bf77bab159 tank 1
```

We also track the number of temp recordings made by each individual
sensor, the number of temp recordings posted to a tank

## Usage

Primitive so far.  Seek something dockerized and cronnish soon.

```
watch -n 10 "cargo run"
```
