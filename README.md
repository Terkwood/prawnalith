# Small Scale Prawn Farming Software

Various programs to manage sensor data from a freshwater prawn grow, and make it available from anywhere in the world, via a minimal UI.

![Prawn Devouring Plankton](https://pbs.twimg.com/media/DpPZJWwUUAAZNEx.jpg:small)

![Networked LED Goodness](demo.gif)

We currently support the display of temperature and pH readings onto small LED matrix screens attached to inexpensive ESP8266 microcontrollers.

Temperature and pH levels are read continuously from the prawns' tank using a DS18B20 submersible temperature sensor, and a SEN 0169 pH meter.

## Resources 

- Background: https://twitter.com/Terkwood/status/1044992354178994178?s=19
- pH sensor wiring and example code: https://www.element14.com/community/docs/DOC-89396/l/arduino-projects-for-arduino-day-water-and-irrigation-projects-part-2

- LED with scrolling text tutorial: https://gustmees.wordpress.com/2018/04/08/first-steps-with-the-arduino-uno-r3-maker-makered-coding-scrolling-text-with-8x8-led-matrix/amp/?__twitter_impression=true
- Cross compiling rust for armv7 from MacOSX using vagrant: https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b
- Cross compiling OpenSSL: https://assil.me/2017/09/30/cross-compile-openssl-arm-zynq.html

## InfluxDB query examples

### Create continuous query to average out 10-minute data

10-min average of all data

```
CREATE CONTINUOUS QUERY mean_all_10m ON schema BEGIN SELECT mean(*) INTO schema.unlimited.mean_all_10m FROM schema.autogen.mqtt_consumer GROUP BY time(10m) END
```

## License

Licensed under either of

 * Apache License, Version 2.0, [LICENSE_APACHE](LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
 * MIT license [LICENSE_MIT](LICENSE_MIT) or http://opensource.org/licenses/MIT

at your option.

### Contribution



Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
