# Temp tracker utility

Listens for temperature-related sensor reports.  Creates an entry in the
Redis `<namespace>/sensors/temp` set if the reporting sensor doesn't exist.  Creates a stubbed hash which the user can fill out in order to associate tank, manufacturing data, etc.

## On temp sensor report

This is useful for sensors generating temperature data.

Such data comes into an MQTT topic looking like this:

```
{ "device_id": <hex>, "temp_f", "temp_c" }
```

This utility first checks to see whether the temp sensor is 
already known to us.

```
SISMEMBER <namespace>/sensors/temp
```

If it isn't, it will create the following type of stub record
for the temp sensor based on a UUID V5 ID conversion:

```
HMSET <namespace>/sensors/temp/<uuid_v5_id> start_date <epoch>
```

The operator is encouraged to later amend the hash to include
a helpful reference to the tank which the sensor serves, so
that the LED status utility can properly format messages.

```
HSET <namespace>/sensors/temp/<uuid_v5_id> tank 0
```

## Building paho-mqtt-rs

### VERY VERY IMPORTANT

- NUMBER ONE
- MOST IMPORTANT
- **DETAIL** **EVER**

Look at https://github.com/eclipse/paho.mqtt.rust/compare/new-build#diff-98fc2489a3a7b302dbce61ba412f464eR61

You should hack your own paho-mqtt-sys/build.rs to look like:

```
fn link_lib() -> &'static str {
    "paho-mqtt3a-static"
}
```

_This will prevent SSL from being used._   It's a horrible idea.  It works.

Carry on.  At this point, from the `paho.mqtt.rust` repo, you can consume in one tty:

```
cargo run --example sync_consume
```

And publish in another:

```
cargo run --example sync_publish
```

### Madness that can't be accounted for

You should probably build this on the pi itself,
and you'll need to install OpenSSL 1.0.1 on the
system in order to achieve universal harmony etc ☯️.

https://assil.me/2017/09/30/cross-compile-openssl-arm-zynq.html

Oh and hey.  You need to build **paho C client** using
the openssl search param.  https://github.com/eclipse/paho.mqtt.c

See see https://github.com/eclipse/paho.mqtt.cpp/issues/136#issuecomment-355280926

```
git clone https://github.com/eclipse/paho.mqtt.c
cd paho.mqtt.c
cmake -DOPENSSL_SEARCH_PATH=/home/pi/openssl_1_0_1   # this directory holds output from follow above guide
make
sudo make install

# Don't forget C++ portion of paho client!
cd ..
git clone https://github.com/eclipse/paho.mqtt.cpp
cd paho.mqtt.cpp
cmake -DPAHO_WITH_SSL=TRUE -DPAHO_BUILD_DOCUMENTATION=FALSE -DPAHO_BUILD_STATIC=TRUE -DPAHO_BUILD_SHARED=FALSE \
        -DPAHO_MQTT_C_PATH=../paho.mqtt.c/ -DPAHO_MQTT_C_LIB=../paho.mqtt.c/src/libpaho.mqtt3as-static.a ${CROSS_COMPILE_ARG}
make
sudo make install

# refreshing the libs
sudo ldconfig
```

OH AND ALSO!
```
export LDFLAGS="-L/home/pi/openssl_1_0_1/lib/"
export LD_LIBRARY_PATH="/home/pi/openssl_1_0_1/lib/"
export CPPFLAGS="-I/home/pi/openssl_1_0_1/include"
```

MAYBE NONE OF THAT WORKED AND YOU TRIED INSTALL THIS:
https://packages.debian.org/search?keywords=libssl1.0-dev

```
# OH AND IF YOU BUILD PAHO C CLIENT
# WHICH YOU MUST
# DO THIS:
export CFLAGS="-DPAHO_BUILD_STATIC=TRUE"
```

_*AND MAKE SURE YOU HAVE YOUR CARGO CONFIG FILE NICE & TIDY*_

```
cat >>~/.cargo/config <EOF
[target.armv7-unknown-linux-gnueabihf.openssl]
libdir = "/home/pi/openssl_1_0_1/lib"
include = "/home/pi/openssl_1_0_1/bin"
```
