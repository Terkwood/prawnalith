# InfluxDB knowledge dump

We use influxdb to keep track of average temp & pH values over a 10 minute interval.  We limit the size of the initial, fine-grained data ingest, so that we don't waste space on our Raspberry Pi and its (!) microSD card.  

_WARNING: WILD SPECULATION AHEAD_

We've found that letting influxdb corpus grow beyond 250MB on disk will risk a "high RAM incident" for influxdb on boot.  Raspberry Pi 3 B+ has only 1GB of RAM.

## InfluxDB setup

It's important to constrain the shard group duration for fine-grained readings to something lower than the default (seven days).  pH sensors emit readings more than once per second, and we don't need to store a large history of those in influxdb.

First, create a database which stores your initial readings for only a short period of time.

```sql
CREATE DATABASE prawnalith WITH DURATION 6h SHARD DURATION 1h NAME prawnalith_fine_grained
```

Next, create a retention policy where the aggregated data will live.

```sql
CREATE RETENTION POLICY "unlimited" ON "prawnalith" DURATION INF REPLICATION 1
```

Then, create a continuous query to average out 10-minute data into a separate table.  This will be assigned to the `unlimited` RP.

```sql
CREATE CONTINUOUS QUERY mean_all_10m ON prawnalith BEGIN SELECT mean(*) INTO prawnalith.unlimited.mean_all_10m FROM prawnalith.prawnalith_fine_grained.mqtt_consumer GROUP BY time(10m) END
```

### Trivia

Query recent data

```sh
influx -database 'prawnalith' -execute 'select * from mqtt_consumer order by time DESC LIMIT 10'
```