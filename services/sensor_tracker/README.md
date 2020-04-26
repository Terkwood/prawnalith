# Sensor tracker

## Purpose

This utility listens for temperature- and pH-related sensor reports.

It is primarily used to update the Redis data structures related to individual _tanks_.  Each tank is tracked such that it contains its most recent temperature and pH report.

Additionally, it creates an entry in the Redis `<namespace>/sensors/<temp_or_ph>` set if the reporting sensor doesn't exist.  Creates a stubbed hash which the user can fill out in order to associate tank, manufacturing data, etc.

## On sensor report

This is useful for sensors generating temperature and/or pH data.

Such data might come into an MQTT topic looking like this:

```json
{ "device_id": <hex>, "temp_f": 81.71, "temp_c": 23.45, "ph": 7.77, "ph_mv": 453.05 }
```

If the device hasn't ever been tracked, it will create the following type of stub record with an internal device ID.  The internal device ID is a (namespaced) UUID V5:

```text
HMSET <namespace>/sensors/<temp_or_ph>/<uuid_v5_id> create_time <epoch>
```

The operator is encouraged to later amend the hash to include
a helpful reference to the area which the sensing device serves, so
that the LED status utility can properly format messages.

```text
HSET <namespace>/sensors/<temp_or_ph>/<device_internal_id> tank 0
```

## Docker builds

See `build.sh` and `run.sh` for entry points.

### Sample redis records

#### temp sensor

`> hgetall namespace/devices/aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa`

```text
 1) "create_time"
 2) "1540598539"
 3) "ext_device_id"
 4) "aaaaaaaa090000aa"
 5) "temp_update_count"
 6) "434817"
 7) "temp_f"
 8) "81.39"
 9) "temp_c"
10) "27.44"
11) "temp_update_time"
12) "1541057567"
13) "tank"
14) "1"
```

#### pH sensor

`> hgetall namespace/devices/aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa`

```text
 1) "low_ph_ref"
 2) "4.00"
 3) "low_mv"
 4) "357.71"
 5) "hi_ph_ref"
 6) "7.03"
 7) "hi_mv"
 8) "441.01"
 9) "ph_update_count"
10) "601570"
11) "ph"
12) "7.84"
13) "ph_mv"
14) "464.21"
15) "ph_update_time"
16) "1541082833"
17) "tank"
18) "1"
19) "ext_device_id"
20) "286cbc98090000bd"
```

#### area counter

`> get namespace/areas`

```text
"1"
```

#### area hash

`> hgetall namespace/areas/1`

```text
hgetall namespace/tanks/1
 1) "temp_f"
 2) "81.16"
 3) "temp_c"
 4) "27.31"
 5) "temp_update_time"
 6) "1541082869"
 7) "temp_update_count"
 8) "683225"
 9) "ph"
10) "7.84"
11) "ph_mv"
12) "464.21"
13) "ph_update_time"
14) "1541082869"
15) "ph_update_count"
16) "601573"
```
