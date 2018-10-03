// Serial sommunication test demonstrating that an Arduino Uno
// can talk to an ESP8266 by connecting the RX/TX pins of the ESP
// to digital IO pins on the arduino.

// First, load `../esp8266_receiver/esp8266_receiver.ino`
// onto an ESP8266.  

// Then load this file onto an Arduino Uno.

// See this directory for fritzing diagrams
// which demonstrate the wiring.
#include <SoftwareSerial.h>

#define RX 8 // Wire this to Tx Pin of ESP8266
#define TX 9 // Wire this to Rx Pin of ESP8266

SoftwareSerial ESP8266 (RX, TX);

void setup() {
  Serial.begin(9600);

  // SoftwareSerial demands 9600.  Respect the baud magic.
  ESP8266.begin(9600); 
  delay(1000);
  Serial.println("Setup complete");
}

void loop() {
  if (ESP8266.available() > 0) { 
    Serial.write(ESP8266.read()); 
  }
}
