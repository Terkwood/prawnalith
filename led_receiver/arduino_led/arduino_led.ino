// Thanks to Chilli_Paste for working out a reliable receiver
// See https://forum.arduino.cc/index.php?topic=514970.0
#include <SoftwareSerial.h>
#define  RX 10   // This digital IO pin on Arduino connects to RX pin of ESP
#define  TX 11   // This digital IO pin on Arduino connects to TX pin of ESP
SoftwareSerial esp8266(RX, TX);

// for some reason this likes being 192 much more than it likes being 256 (the value on the other side)
const byte serial_push_size = 192;
char received_chars[serial_push_size];

boolean new_data = false;

void setup() {
  Serial.begin(9600);
  esp8266.begin(9600);
}

int last_printed_ms = 0;
int print_freq_ms = 5000;

void loop() {
  recv();

  int now = millis();
  if (now > last_printed_ms + print_freq_ms) {
    Serial.println(received_chars);
    last_printed_ms = now;
  }
} 

// receives messages which are guaranteed to have
// { start and end markers }
void recv() {
  static boolean recv_in_progress = false;
  static byte ndx = 0;
  char start_marker = '{';
  char end_marker = '}';
  char rc;
  while (esp8266.available() > 0 && new_data == false) {
    rc = esp8266.read();
    if (recv_in_progress == true) {
      if (rc != end_marker) {
        received_chars[ndx] = rc;
        ndx++;
        if (ndx >= serial_push_size) {
          ndx = serial_push_size - 1;
        }
      }
      else {
        received_chars[ndx] = '\0'; // terminate the string
        recv_in_progress = false;
        ndx = 0;
        new_data = true;
      }
    }
    else if (rc == start_marker) {
      recv_in_progress = true;
    }
  }
  new_data = false;
}


