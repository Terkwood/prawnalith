# LED status helper

Periodically generates a human-readable, full system report, which will
be shown on small LED matrix displays.

## Initial usage

The LED status helper checks all tanks for their current temperature
and pH values, and emits a report formatted as follows:

```
#1 82.30F 7.1pH #2 83.11F 6.9pH
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
HMGET prawnalith/tanks/1 temp ph
HMGET prawnalith/tanks/2 temp ph
```

## Usage

Primitive so far.  Seek something dockerized and cronnish soon.

```
watch -n 10 "cargo run"
```
