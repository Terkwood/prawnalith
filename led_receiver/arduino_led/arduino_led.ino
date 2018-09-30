#include <SoftwareSerial.h>
#define RX 10
#define TX 11

SoftwareSerial esp8266(RX,TX); 

// ATTRIBUTION LINK: http://forum.arduino.cc/index.php?topic=396450.0
#define SERIAL_PUSH_SIZE 256
char receivedChars[SERIAL_PUSH_SIZE];   // an array to store the received data
bool newData = false;

void recvWithEndMarker() {
    static byte ndx = 0;
    char endMarker = '\n';
    char rc;
    newData = false;
    while (esp8266.available() > 0 && newData == false /*&& ndx < SERIAL_PUSH_SIZE*/) {
        rc = esp8266.read();

        if (rc != endMarker) {
            receivedChars[ndx] = rc;
            ndx++;
            if (ndx >= SERIAL_PUSH_SIZE) {
                ndx = SERIAL_PUSH_SIZE - 1;
            }
        }
        else {
            receivedChars[ndx] = '\0'; // terminate the string
            ndx = 0;
            newData = true;
        }

        
    }
    Serial.println(receivedChars);
}


void setup() {
  Serial.begin(9600);
  esp8266.begin(9600);
  // clear the esp buffer initially
  while (esp8266.available() > 0) {
    esp8266.read();
  }
}

void loop() {
  recvWithEndMarker();
  delay(1000);
}
