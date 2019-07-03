#include <DHTesp.h>
#include <WiFiClient.h>
#include <ESP8266WiFi.h>
// TODO did we alter MQTT_KEEPALIVE ? See ph_temp_sensor_esp8266
#include <PubSubClient.h>

// WIFI SETUP
const char* ssid = "ssid";
const char* password = "password";
WiFiClient wifi_client;

// MQTT SETUP
const char* mqtt_broker = "broker";
const char* mqtt_topic  = "namespace/sensors";
PubSubClient mqtt_client(wifi_client);

#define MQTT_MESSAGE_SIZE 128
char mqtt_message[MQTT_MESSAGE_SIZE];

// HUMIDITY & TEMP SENSOR
DHTesp dht;

const int MEASUREMENT_FREQ_MS = dht.getMinimumSamplingPeriod() * 2;

void setup()
{
  Serial.begin(115200);
  Serial.println();
  Serial.println(ARDUINO_BOARD);
  Serial.println("Status\tHumidity (%)\tTemperature (C)\t(F)\tHeatIndex (C)\t(F)");


  dht.setup(D4, DHTesp::DHT11); // Connect DHT sensor to GPIO 4
}

void loop()
{
  delay(MEASUREMENT_FREQ_MS);

  float humidity = dht.getHumidity();
  float temperature = dht.getTemperature();

  Serial.print(dht.getStatusString());
  Serial.print("\t");
  Serial.print(humidity, 1);
  Serial.print(" H%");
  Serial.print("\t\t");
  Serial.print(temperature, 1);
  Serial.print(" C");
  Serial.print("\t\t");
  Serial.print(dht.toFahrenheit(temperature), 1);
  Serial.print(" F");
  Serial.print("\t\t");
  Serial.print(dht.computeHeatIndex(temperature, humidity, false), 1);
  Serial.print(" H%C");
  Serial.print("\t\t");
  Serial.print(dht.computeHeatIndex(dht.toFahrenheit(temperature), humidity, true), 1);
  Serial.println(" H%F");
} 

