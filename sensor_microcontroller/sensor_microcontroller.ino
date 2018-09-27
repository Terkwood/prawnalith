#include <OneWire.h>
#include <DallasTemperature.h>
#include <WiFiClient.h>
#include <ESP8266WiFi.h>
#include <PubSubClient.h>

// WIFI SETUP
const char* ssid = "NOPE"; 
const char* password = "NADA";



// MQTT SETUP
const char* mqtt_broker = "NUH-UH";
const char* mqtt_topic  = "test/test";
WiFiClient wifi_client;
PubSubClient mqtt_client(wifi_client);
#define MQTT_MESSAGE_SIZE 128
char mqtt_message[MQTT_MESSAGE_SIZE];




// DS18B20 SETUP
#define ONE_WIRE_PIN D3
#define MAX_ONE_WIRE_DEVICES 15

OneWire one_wire(ONE_WIRE_PIN);
DallasTemperature DS18B20(&one_wire);

int temp_sensor_count;

DeviceAddress device_addresses[MAX_ONE_WIRE_DEVICES];

// millis time of last measurement
long last_temp_measurement_ms; 
// how often to measure temperature
const int temp_measurement_freq_ms = 5000;

// Various resolutions are available for DS18B20 temp sensor
// See https://cdn-shop.adafruit.com/datasheets/DS18B20.pdf
/*
 * Mode    Resolution  Conversion time
 * 9 bits  0.5°   C     93.75 ms
 * 10 bits 0.25°  C    187.5  ms
 * 11 bits 0.125° C    375    ms
 * 12 bits 0.0625°C    750    ms
 */
const int DS18B20_RESOLUTION = 12;



String DeviceIdToString(DeviceAddress deviceAddress)
{
   String s;
 
   for (uint8_t i = 0; i < 8; i++) {
     if (deviceAddress[i] < 16) s += "0";  // zero pad the address if necessary
     s += String(deviceAddress[i], HEX);
   }
   
   return s;
}

void InitMQTT() {
  // We'll need to randomly generate a client ID later.
  randomSeed(micros());

  mqtt_client.setServer(mqtt_broker, 1883);
}

// Thanks to https://github.com/knolleary/pubsubclient/blob/master/examples/mqtt_esp8266/mqtt_esp8266.ino
void ConnectMQTT() {
  // Loop until we're connected
  while (!mqtt_client.connected()) {
    Serial.print("Connecting to MQTT broker...");
    // Create a random client ID
    String clientId = "esp8266_";
    clientId += String(random(0xffff), HEX);
    
    // Attempt to connect
    if (mqtt_client.connect(clientId.c_str())) {
      Serial.println("MQTT connected");
    } else {
      Serial.print("MQTT connection failed, rc=");
      Serial.print(mqtt_client.state());
      Serial.println(" ...trying again in 5 seconds");
      
      delay(5000);
    }
  }
}

void InitWifi() {
  WiFi.begin(ssid, password);
  
  while (WiFi.status() != WL_CONNECTED) {
    delay(333);
    Serial.println("...waiting for Wi-Fi connection...");
  }

  Serial.println("");
  Serial.print("Wireless network SSID: ");
  Serial.println(ssid);
  Serial.print("Local IP address:      ");
  Serial.println(WiFi.localIP());
  Serial.println("");
}

void InitDS18B20() {
  DS18B20.begin();

  Serial.println("");
  Serial.print("DS18B20 parasite power:  ");
  if (DS18B20.isParasitePowerMode()) { 
    Serial.println("ENABLED");
  } else {
    Serial.println("DISABLED");
  }
  
  temp_sensor_count = DS18B20.getDeviceCount();
  Serial.print("DS18B20 sensors present: ");
  Serial.print(temp_sensor_count);
  Serial.println("");
  Serial.println("");

  last_temp_measurement_ms = millis();
  DS18B20.requestTemperatures();

  for(int i = 0; i < temp_sensor_count; i++){
    Serial.print("Device ");
    Serial.print(i, DEC);
    Serial.println(":");
    
    if(DS18B20.getAddress(device_addresses[i], i)){
      Serial.print("    Address: ");
      Serial.println(DeviceIdToString(device_addresses[i]));
    } else {
      Serial.println("    WARNING!  This device has no address.  Please verify that it is connected properly.");
    }

    DS18B20.setResolution(device_addresses[i], DS18B20_RESOLUTION);
    
    Serial.print("    Resolution set to: ");
    Serial.print(DS18B20.getResolution(device_addresses[i]));
    
    Serial.println("");
  }
  Serial.println("");
  Serial.println("");
}

void setup() {
  Serial.begin(115200);

  InitWifi();

  InitMQTT();

  InitDS18B20();
}

void loop() {
  long now = millis();

  if (!mqtt_client.connected()) {
    ConnectMQTT();
  }
  mqtt_client.loop();


  if (temp_measurement_freq_ms + last_temp_measurement_ms < now) {
    for (int i = 0; i < temp_sensor_count; i++) {
      float celsius_reading = DS18B20.getTempC(device_addresses[i]);
      float fahrenheit_reading = DS18B20.getTempF(device_addresses[i]);

      // publish formatted message to MQTT topic
      snprintf(
        mqtt_message,
        MQTT_MESSAGE_SIZE,
        "{ \"device_id\": \"%s\", \"temp_c\": %.2f, \"temp_f\": %.2f }",
        DeviceIdToString(device_addresses[i]).c_str(),  // snprintf wants a const char*
        celsius_reading,
        fahrenheit_reading
      );
      mqtt_client.publish(mqtt_topic, mqtt_message);
      
      // let there be terminal spam
      Serial.print(DeviceIdToString(device_addresses[i]));
      Serial.print(": ");
      Serial.print(celsius_reading);
      Serial.print("C ");
      Serial.print(fahrenheit_reading);
      Serial.print("F ");
      Serial.println("");
    }

    // If you're using a low resolution (quicker measurement times),
    // then you can probably set this to false.
    DS18B20.setWaitForConversion(true);
    
    DS18B20.requestTemperatures();
    
    last_temp_measurement_ms = millis();
  }
}
