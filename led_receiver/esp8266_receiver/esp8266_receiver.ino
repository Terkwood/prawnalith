// FLASH IS LIFE
// FLASH WITH CONFIDENCE AFTER READING THIS ARTICLE:
//     https://tttapa.github.io/Pages/Arduino/ESP8266/Flashing/Flashing-With-an-Arduino.html

#include <WiFiClient.h>
#include <ESP8266WiFi.h>
#include <PubSubClient.h>

const char* ssid = "SSID";      
const char* password = "PASS";

const char* mqtt_broker = "BROKER_HOST";
const char* mqtt_topic = "prawnalith/led"; 



const char* p_init_complete =     "# INIT_COMPLETE";
const char* p_wifi_connected =    "# WIFI_CONNECTED ";
const char* p_connect_mqtt   =    "# CONNECT_MQTT";
const char* p_mqtt_connected =    "# MQTT_CONNECTED";
const char* p_mqtt_failed_retry = "# MQTT_FAILED_RETRY ";
const char* p_mqtt_subscribed =   "# MQTT_SUBSCRIBED ";


// LINE PROTOCOL
//
// Push data.  The ESP8266 will push lots of data to the
// Arduino side, most of which will be summary messages.
// They are expected to be a single line, and are pre-formatted
// to fit the LED.
// 
//     { Tank 0 Temp 81.50°F pH 7.01 Tank 1 Temp 82.03°F pH 6.89 }
// 
// Initialization spam and connectivity metadata.
//
// In terms of the LED display processing,
// if a line starts with #, ignore it. In practice,
// the Arduino side doesn't need to implement any filtering,
// as long as it waits 10-15 seconds before polling the
// Serial connection.
//
// ESP will broadcast these messages on startup,
// and MQTT-related connection messages may flow
// during the process execution.
//
// Metadata message formats.
//
// This message shows the SSID of the access
// point and local IP after a successful connection
// is made.
// 
//     # WIFI_CONNECTED <our_ap> <our_ip>
//
// This message shows that we are currently attempting
// to connect to the MQTT broker.
// 
//     # CONNECT_MQTT
//
// This message shows that we successfully connected to the 
// MQTT broker.  These messages can appear at any time
// during the program's execution, so you need to always
// be sure to throw away hash-prefixed (#) lines.
//
//     # MQTT_CONNECTED
//
// This message shows that we did not connect to the broker,
// and we are retrying.  It signals the MQTT client state.
//
//     # MQTT_FAILED_RETRY <client_state>
//
// This message shows topic subscription success in MQTT.
// For now you will only see one of these.
//     
//     # MQTT_SUBSCRIBED <topic>
//
// This message shows that initialization is complete.
// Once this is received, you can expect freshly-formatted
// messages for the LED.
// 
//     # INIT_COMPLETE
//     


WiFiClient wifi_client;
PubSubClient mqtt_client(wifi_client);

#define SERIAL_PUSH_SIZE 256

char push_data[SERIAL_PUSH_SIZE];
bool push_ready = false;

void SubscribeCallback(char* topic, byte* payload, unsigned int payload_length) {
  push_ready = false;
  memcpy(push_data, payload, payload_length);
  push_ready = true;
}


void InitWifi() {
  // Connect to WiFi network
  
  WiFi.begin(ssid, password);
   
  while (WiFi.status() != WL_CONNECTED) {
    delay(25);
  }

  Serial.print(p_wifi_connected);
  Serial.print(" ");
  Serial.print(ssid);
  Serial.println(WiFi.localIP());

  Serial.println(p_init_complete);
}
 
void InitMQTT() {
  // We'll need to randomly generate a client ID later.
  randomSeed(micros());

  mqtt_client.setServer(mqtt_broker, 1883);
  mqtt_client.setCallback(SubscribeCallback);
}


void ConnectMQTT() {
  // Signal across the line that we're trying to
  // connect.
  Serial.println(p_connect_mqtt);

  // Loop until we're connected
  while (!mqtt_client.connected()) {
    // Create a random client ID
    String clientId = "espLED_";
    clientId += String(random(0xffff), HEX);
    
    // Attempt to connect
    if (mqtt_client.connect(clientId.c_str())) {
      Serial.println(p_mqtt_connected);
      mqtt_client.subscribe(mqtt_topic);
      Serial.print(p_mqtt_subscribed);
      Serial.println(mqtt_topic);
    } else {
      Serial.print(p_mqtt_failed_retry);
      Serial.println(mqtt_client.state());
      
      delay(5000);
    }
  }
}


  
void setup() {
  // SoftwareSerial library (arduino side) doesn't want higher baud than 9600
  Serial.begin(9600);
  
  InitWifi();

  InitMQTT();
}


int last_push_ms;
const int push_freq_ms = 5000;

void loop() { 
  if (!mqtt_client.connected()) {
    ConnectMQTT();
  }

  mqtt_client.loop();
  
  int now = millis();
  if (now > last_push_ms + push_freq_ms) {
    Serial.println(push_data);
    last_push_ms = now;
  }
}