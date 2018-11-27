# The Prawnalith: Small-Scale Aquarium Management

Various programs to manage sensor data from a freshwater prawn grow, and make it available from anywhere in the world, via a minimal UI.

![Get a Skillet... Prawnto](https://gist.githubusercontent.com/Terkwood/c37c50d41bcc84b409eeaa555f788ed0/raw/ec42ccbdee0c3f518400253b5e1270f5488f4f1c/prawnto.gif)

We currently support the display of temperature and pH readings onto small LED matrix screens attached to inexpensive ESP8266 microcontrollers.

Temperature and pH levels are read continuously from the prawns' tank using a DS18B20 submersible temperature sensor, and a SEN 0169 pH meter.

## Features

Implements a gated frontend in Rust/webassembly which renders temp & pH levels of tanks.  This can be used to monitor tank levels from afar. 

Provides support for continuous sensing of pH levels using a SEN0169 pH sensor linked to an ESP8266 microcontroller.

Provides basic temperature readings for a given aquarium, using a DS18B20 temp sensor hooked to an ESP8266 microcontroller. 

Displays temp and pH readings on remotely networked LED arrays.

Provides a "pH Reference Calibration" webservice which allows pH sensors to query their high & low pH calibration values in millivolts when coming online.

Includes several docker images and config which can be hosted on a Raspberry Pi 3 B+. These include:

- ph & temp sensor tracker (listens for temp & pH updates provided by ESP8266 over MQTT and writes them to database)
- led status helper (polls database for the temp & pH of individual tanks, and pushes a formatted message to MQTT; this can be read by LED microcontroller units)
- redis update aggregator (pushes temp & pH level updates to google cloud/pub sub)
- grafana setup in docker compose
- mosquitto setup in docker compose
- influx setup on docker compose (stores data queried by grafana)
- redis setup in docker compose (stores miscellaneous data, including tank status for LED display)

Includes basic examples of pH meter readings using an Arduino, and serial communication between an Arduino wired to an ESP8266.

Includes configuration for a google container OS instance.  Runs a small Rocket.rs webserver which can broker temp & pH data requests for the frontend and receive updates to these levels from google cloud pub/sub push mechanism.

## Resources 

- Background: https://twitter.com/Terkwood/status/1044992354178994178?s=19
- pH sensor wiring and example code: https://www.element14.com/community/docs/DOC-89396/l/arduino-projects-for-arduino-day-water-and-irrigation-projects-part-2

- LED with scrolling text tutorial: https://gustmees.wordpress.com/2018/04/08/first-steps-with-the-arduino-uno-r3-maker-makered-coding-scrolling-text-with-8x8-led-matrix/amp/?__twitter_impression=true
- Cross compiling rust for armv7 from MacOSX using vagrant: https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b
- Cross compiling OpenSSL: https://assil.me/2017/09/30/cross-compile-openssl-arm-zynq.html


## License

Licensed under either of

 * Apache License, Version 2.0, [LICENSE_APACHE](LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
 * MIT license [LICENSE_MIT](LICENSE_MIT) or http://opensource.org/licenses/MIT

at your option.

### Contribution

Thank you for your interest!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
