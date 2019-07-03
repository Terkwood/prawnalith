#include <DHTesp.h>
#include <WiFiClient.h>
#include <ESP8266WiFi.h>
// NB.  We have altered the `#define MQTT_KEEPALIVE 15` in PubSubClient.h
//      to be set as `#define MQTT_KEEPALIVE 60`.  This is done in an effort
//      to allow the time-intensive scrolling process not take so long that
//      it exceeds the default keepalive in PubSubClient.h.
//      If you're using the Arduino IDE, this can usually be found in
//      ~/Documents/Arduino/libraries/PubSubClient/PubSubClient.h
//      WE ALSO MODIFY MQTT MAX PACKET SIZE
//      `#define MQTT_MAX_PACKET_SIZE 256`
#include <PubSubClient.h>


// WARNING! YOU MUST REPLACE THE EXAMPLE VALUE BELOW
const char* DEVICE_ID = "b1c8ae88-8622-415f-951e-27a21888fe19";


// WIFI CONFIGURABLE CONSTANTS
const char* ssid = "ssid";
const char* password = "password";


// MQTT CONFIGURABLE CONSTANTS
const char* mqtt_broker = "broker";
const int mqtt_port = 1883;
const char* mqtt_topic  = "namespace/sensors";


// WIFI VARS
WiFiClient wifi_client;

// PUBSUB VARS
PubSubClient mqtt_client(wifi_client);
// WARNING WARNING WARNING 
// WARNING WARNING WARNING 
//      PubSubClient.h must be modified to support the next setting!
//      ~/Documents/Arduino/libraries/PubSubClient/PubSubClient.h
//      #define MQTT_MAX_PACKET_SIZE 256
#define MQTT_MESSAGE_SIZE 256
char mqtt_message[MQTT_MESSAGE_SIZE];
#define MQTT_RETRY_MS 5000

// HUMIDITY & TEMP SENSOR VARS
DHTesp dht;

const int MEASUREMENT_FREQ_MS = dht.getMinimumSamplingPeriod() * 2;

void setup_wifi(void) {
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

void setup_dht_sensor(void) {
  dht.setup(D4, DHTesp::DHT11); // Connect DHT sensor to GPIO 4
}

void setup_mqtt(void) {
  // We'll need to randomly generate a client ID later.
  randomSeed(micros());

  mqtt_client.setServer(mqtt_broker, mqtt_port);

  Serial.print("Publishing to broker ");
  Serial.print(mqtt_broker);
  Serial.print(":");
  Serial.print(mqtt_port);
  Serial.print(" with topic ");
  Serial.println(mqtt_topic);
}


// Thanks to https://github.com/knolleary/pubsubclient/blob/master/examples/mqtt_esp8266/mqtt_esp8266.ino
void connect_mqtt(void) {
  // Loop until we're connected
  while (!mqtt_client.connected()) {
    Serial.print("Connecting to MQTT broker...");
    // Create a random client ID
    String clientId = "dht11_";
    clientId += String(random(0xffff), HEX);

    // Attempt to connect
    if (mqtt_client.connect(clientId.c_str())) {
      Serial.println("MQTT connected");
    } else {
      Serial.print("MQTT connection failed, rc=");
      Serial.print(mqtt_client.state());
      Serial.println(" ...trying again in 5 seconds");

      delay(MQTT_RETRY_MS);
    }
  }
}


void setup(void)
{
  Serial.begin(115200);

  setup_wifi();

  setup_mqtt();

  setup_dht_sensor();

  Serial.print("prawnalith device_id ");
  Serial.println(DEVICE_ID);
  Serial.println();
  Serial.println(ARDUINO_BOARD);
}

void loop(void)
{
  delay(MEASUREMENT_FREQ_MS);

  if (!mqtt_client.connected()) {
    connect_mqtt();
  }
  mqtt_client.loop();

  float humidity = dht.getHumidity();
  float temperature = dht.getTemperature();
  
  // publish formatted message to MQTT topic
  snprintf(
    mqtt_message,
    MQTT_MESSAGE_SIZE,
    "{ \"device_id\": \"%s\", \"status\": \"%s\", \"temp_c\": %.2f, \"temp_f\": %.2f, \"humidity\": %.2f, \"heat_index_c\": %.2f, \"heat_index_f\": %.2f }",
    DEVICE_ID,  // snprintf wants a const char*
    dht.getStatusString(),
    temperature,
    dht.toFahrenheit(temperature),
    humidity,
    dht.computeHeatIndex(temperature, humidity, false),
    dht.computeHeatIndex(dht.toFahrenheit(temperature), humidity, true)
  );
  
  bool publish_result = mqtt_client.publish(mqtt_topic, mqtt_message);

  if (!publish_result) {
    Serial.println("MQTT publish failed.  Message too large?  Check that PubSubClient.h has the following workaround:");
    Serial.println("#define MQTT_MAX_PACKET_SIZE 256");
  }

  Serial.println(mqtt_message);
} 
