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

You need the static binary for `libpaho-mqtt3as` v1.2.1 available
on the build machine when building `paho-mqtt-rs`.

We used `vagrant` and installed the following libs:

```
sudo apt-get update
sudo apt-get install -y  build-essential                 \
                    libssl-dev                      \
                    gcc                             \
                    make                            \
                    cmake                           \
                    cmake-gui                       \
                    cmake-curses-gui                \
                    automake                        \
                    autoconf                        \
                    libtool                         \
                    doxygen                         \
                    graphviz                        \
                    git                             \
                    gcc-arm-linux-gnueabihf         \
                    g++-arm-linux-gnueabihf

export PROJECT_DIR=/tmp/build_deps
mkdir -p $PROJECT_DIR
cd $PROJECT_DIR
git clone https://github.com/eclipse/paho.mqtt.c
cd paho.mqtt.c
git checkout v1.2.1                       # This is important, friends! 🙃
cmake -DPAHO_WITH_SSL=TRUE -DPAHO_BUILD_DOCUMENTATION=FALSE -DPAHO_BUILD_STATIC=TRUE -DPAHO_BUILD_SAMPLES=TRUE ${CROSS_COMPILE_ARG}
make
sudo make install
```

You also need to make sure you have ARM libs for OpenSSL available on your vagrant box.  Take a look at https://assil.me/2017/09/30/cross-compile-openssl-arm-zynq.html.

Then, e.g:

```
export ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_DIR=~/cross-openssl/openssl
export ARMV7_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_LIB_DIR=~/cross-openssl/openssl
```

You should then be able to run

```
cargo build --target=armv7-unknown-linux-gnueabihf
```

And generate a working binary for ARMv7.

You can see some hints about this in https://github.com/eclipse/paho.mqtt.cpp/issues/136#issuecomment-355280926.

Thank you, Paho team & contributors! 🙏🏼
