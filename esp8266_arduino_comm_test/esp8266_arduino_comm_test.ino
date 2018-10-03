// communication test
#include <SoftwareSerial.h>

#define RX 8 // Wire this to Tx Pin of ESP8266
#define TX 9 // Wire this to Rx Pin of ESP8266

SoftwareSerial ESP8266 (RX, TX);

void setup() {
  Serial.begin(9600);
  ESP8266.begin(9600); // SoftwareSerial demands 9600.  Respect the 9600 baud magic.
  delay(1000);
  Serial.println("Setup complete");
}

void loop() {
  if (ESP8266.available() > 0) { 
    Serial.write(ESP8266.read()); 
  }
}

