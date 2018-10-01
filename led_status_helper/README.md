# LED status helper

Periodically generates a human-readable, full system report, which will
be shown on small LED matrix displays.

## Initial usage

The LED status helper checks all tanks for their current temperature
and pH values, and emits a report formatted as follows:

```
{ #1 82.30°F 7.1pH #2 83.11°F 6.9pH }
```

The curly-braces are `start_of_message` and `end_of_message` characters,
and must be present in order for the LED to correctly display the message.

## Redis query pattern

We expect Redis to hold a counter which tells us how many tanks to query.

```
GET prawnalith/tanks
"2"
```

Each tank entry is expected to have a `temp` and `ph` field based
on the most recent reading from its sensor.

```
HMGET prawnalith/tanks/1 temp ph
HMGET prawnalith/tanks/2 temp ph
```
