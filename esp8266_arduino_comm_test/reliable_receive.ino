// Thanks to Chilli_Paste for working out a reliable receiver
// See https://forum.arduino.cc/index.php?topic=514970.0
#include <SoftwareSerial.h>
#define  RX 8   // digital IO pin on Arduino connects to RX pin of ESP
#define  TX 9   // digital IO pin on Arduino connects to TX pin of ESP
SoftwareSerial esp8266(RX, TX);

// for some reason this likes being smaller than 256 (the value on the other side)
const byte serial_push_size = 128;
char received_chars[serial_push_size];

boolean new_data = false;

 
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




void setup(){
  Serial.begin(9600);
  esp8266.begin(9600);

  init_leds();
}


static int last_scroll_ms = 0;
const int scroll_freq_ms = 15000;
void loop(){
    recv();
    
    int now = millis();
    if (now > last_scroll_ms + scroll_freq_ms) {
      strlcpy(scroll_text, received_chars, serial_push_size);
      Serial.print("Display: ");
      Serial.println(scroll_text);
  
      for (int i = 0; i < serial_push_size; i++) {
        load_buffer_long((int) scroll_text[i]);
      }
      last_scroll_ms = millis();
    }
}
