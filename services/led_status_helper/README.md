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

Make sure there is a .env file with data which matches the Config class.

Then simply run with `cargo run` or a compiled binary.

## Cross-compilation

You're advised to cross-compile the ARMv7 executables on something fast,
like a Mac OS X or large linux box, instead of compiling them on a Raspberry
Pi host itself.  You'll save yourself a lot of time this way.

But you'll need to do a bit of setup to get cross-compilation working, 
especially with OpenSSL.

Follow these links:

- https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b
- https://assil.me/2017/09/30/cross-compile-openssl-arm-zynq.html

Note that when you compile a rust project which depends on OpenSSL, you
need to have the version of OpenSSL cross-compiled which is *expected by rust-openssl*.
At the time of this writing, we found that OpenSSL 1.0.1 worked best.

You can select the correct version by inspecting release tags of the OpenSSL project,
e.g. https://github.com/openssl/openssl/releases/tag/OpenSSL_1_0_1t

```
git checkout OpenSSL_1_0_1t
```

