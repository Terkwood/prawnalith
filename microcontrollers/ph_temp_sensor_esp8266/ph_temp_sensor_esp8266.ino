// pH sensor (SEN 0169) and digital temp (DS18B20) on an ESP8266
// pH sensor code adapted from https://www.dfrobot.com/wiki/index.php/Analog_pH_Meter_Pro_SKU:SEN0169

#include <OneWire.h>
#include <DallasTemperature.h>
#include <WiFiClient.h>
#include <ESP8266WiFi.h>

// NB.  We have altered the `#define MQTT_KEEPALIVE 15` in PubSubClient.h
//      to be set as `#define MQTT_KEEPALIVE 60`.  This is done in an effort
//      to allow the time-intensive scrolling process not take so long that
//      it exceeds the default keepalive in PubSubClient.h.
#include <PubSubClient.h>

// WIFI SETUP
const char* ssid = "ssid";
const char* password = "password";

// MQTT SETUP
const char* mqtt_broker = "broker";
const char* mqtt_topic  = "namespace/sensors";

// CALIBRATION HTTP HOST SETUP
const char* calibration_host = "someone_nearby";
const int calibration_port = 8000;

const char* calibration_path = "/sensors/ph/calibration";

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
   Mode    Resolution  Conversion time
   9 bits  0.5°   C     93.75 ms
   10 bits 0.25°  C    187.5  ms
   11 bits 0.125° C    375    ms
   12 bits 0.0625°C    750    ms
*/
const int DS18B20_RESOLUTION = 12;



// PH SENSOR SETUP
#define PRINT_INTERVAL 800
#define PH_SENSOR_PIN A0            // pH meter Analog output to Arduino Analog Input 0

#define PH_SAMPLING_INTERVAL 20
#define PH_ARRAY_LENGTH  40         // Collect this many samples

int ph_array[PH_ARRAY_LENGTH];      // Store sampled values from the sensor reading
int ph_array_index = 0;


float ph_hi_ref = 7.0;    // default
float ph_low_ref = 4.01;  // default reference point for low pH solution
float ph_hi_mv = 441.01;  // default millivolt reading
float ph_low_mv = 357.71; // default

#define PH_FLOOR 7.0

// calculate pH value based on millivolt reading on pH sensor
float ph(float raw_voltage) {
  float ref_diff = ph_hi_ref - ph_low_ref;
  float i = raw_voltage - ph_hi_mv;
  return (i / ((ph_hi_mv - ph_low_mv) / ref_diff)) + PH_FLOOR;
}  




// DS18B20 device ID (an "external device ID" known to the prawnalith)
String DeviceIdToString(DeviceAddress deviceAddress)
{
  String s;

  for (uint8_t i = 0; i < 8; i++) {
    if (deviceAddress[i] < 16) s += "0";  // zero pad the address if necessary
    s += String(deviceAddress[i], HEX);
  }

  return s;
}


// used by PH sensor routines to find the average value in an array of numbers
double ArrayAvg(int* arr, int number) {
  int i;
  int max, min;
  double avg;
  long amount = 0;
  if (number <= 0) {
    Serial.println("Error number for the array to average!/n");
    return 0;
  }
  if (number < 5) { // treat small array as a special case
    for (i = 0; i < number; i++) {
      amount += arr[i];
    }
    avg = amount / number;
    return avg;
  } else {
    if (arr[0] < arr[1]) {
      min = arr[0]; max = arr[1];
    }
    else {
      min = arr[1]; max = arr[0];
    }
    for (i = 2; i < number; i++) {
      if (arr[i] < min) {
        amount += min;      //arr<min
        min = arr[i];
      } else {
        if (arr[i] > max) {
          amount += max;  //arr>max
          max = arr[i];
        } else {
          amount += arr[i]; //min<=arr<=max
        }
      }//if
    }//for
    avg = (double)amount / (number - 2);
  }//if
  return avg;
}


void LoadReferenceCsv(char* token_string) {
  char seps[] = ",";
  char* token;
  float var;
  float input[4];
  int i = 0;
  token = strtok (token_string, seps);
  while (token != NULL)
  {
      sscanf (token, "%f", &var);
      input[i++] = var;
  
      token = strtok (NULL, seps);
  }

  ph_low_ref = input[0];
  ph_low_mv = input[1];
  ph_hi_ref = input[2];
  ph_hi_mv = input[3];

  Serial.printf(
    "Assigned\n\t ph_low_ref %f \n\t ph_low_mv %f \n\t ph_hi_ref %f \n\t ph_hi_mv %f \n\n", 
    input[0], input[1], input[2], input[3]
  );
}


/* Makes an HTTP call to fetch millivolt pH reference values
 * for low (~4.01) and hi (~7.0) pH calibration levels.
 * Equivalent to:
 * `curl http://localhost:8000/sensors/ph/calibration\?ext_id\=aaaaffff000000f0\&device_type\=ph -H "Accept: text/csv"`
 * The response for this will generally look like:

      low_ph_ref,low_mv,hi_ph_ref,hi_mv
      4.00,357.71,7.03,441.01

 * This response is parsed and the values are assigned to the appropriate globals, 
 * for subsequent use when measuring voltage levels with the sensor!
 */
void InitPhRefGlobals(String ext_device_id) {
  Serial.printf("\nConnecting to pH calibration host %s:%d ... ", calibration_host, calibration_port);
  if (wifi_client.connect(calibration_host, calibration_port))
  {
    String payload, data_line;
    String get_request = String("GET ") + calibration_path + "?ext_id=" + ext_device_id + "&device_type=ph"+ " HTTP/1.1\r\n" +
                      "Host: " + calibration_host + ":" + calibration_port + "\r\n" +
                      "Accept: text/csv\r\n" +
                      "Connection: close\r\n" +
                      "\r\n";
    wifi_client.print(get_request);
    Serial.println(get_request);

    /*
     * Typical HTTP response
        
        HTTP/1.1 200 OK
        Content-Type: text/plain; charset=utf-8
        Server: Rocket
        Content-Length: 58
        Date: Sun, 14 Oct 2018 22:33:18 GMT
        
        low_ph_ref,low_mv,hi_ph_ref,hi_mv
        4.00,357.71,7.03,441.01
     
     */
    while (wifi_client.connected()) {
      if (wifi_client.available()) {
          bool looking_for_csv_header = true;
          while (looking_for_csv_header) {
            payload = wifi_client.readStringUntil('\n');
            payload += '\n';
            if (payload == String("low_ph_ref,low_mv,hi_ph_ref,hi_mv\n")) {
              looking_for_csv_header = false;
            }  
          }
          data_line = wifi_client.readStringUntil('\n');
      }
    }
  
    Serial.println("[FOUND CALIBRATION CSV HEADER]");
    Serial.println(payload);
    Serial.println("[DATA LINE]");
    Serial.println(data_line);
    LoadReferenceCsv((char*) data_line.c_str());
    
  } else {
    Serial.println("... connection failed!");
  }

  wifi_client.stop();
}



void InitMQTT(void) {
  // We'll need to randomly generate a client ID later.
  randomSeed(micros());

  mqtt_client.setServer(mqtt_broker, 1883);
}

// Thanks to https://github.com/knolleary/pubsubclient/blob/master/examples/mqtt_esp8266/mqtt_esp8266.ino
void ConnectMQTT(void) {
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

void InitWifi(void) {
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

void InitDS18B20(void) {
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

  for (int i = 0; i < temp_sensor_count; i++) {
    Serial.print("Device ");
    Serial.print(i, DEC);
    Serial.println(":");

    if (DS18B20.getAddress(device_addresses[i], i)) {
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


void setup(void)
{
  Serial.begin(115200);

  // Order is important:  we need to initialize the wifi client
  // before trying to talk to MQTT or call the web server in InitPhRefValues
  InitWifi();

  InitMQTT();

  // Order is important: initialize DS18B20 before trying to access
  // its device ID in the next line.
  InitDS18B20();

  // just use the device ID of the first temp sensor as the identifier for the pH meter
  // it'll work as long as we only ever connect a single temp sensor to these units :-D
  InitPhRefGlobals(DeviceIdToString(device_addresses[0]));
}

float celsius_reading, fahrenheit_reading;


void loop(void)
{
  static unsigned long ph_sampling_time = millis();
  static unsigned long print_time = millis();
  static float ph_value, raw_voltage;

  if (millis() - ph_sampling_time > PH_SAMPLING_INTERVAL)
  {
    ph_array[ph_array_index++] = analogRead(PH_SENSOR_PIN);
    if (ph_array_index == PH_ARRAY_LENGTH) ph_array_index = 0;
    raw_voltage = ArrayAvg(ph_array, PH_ARRAY_LENGTH);
    ph_value = ph(raw_voltage);
    ph_sampling_time = millis();
  }

  long now = millis();

  if (!mqtt_client.connected()) {
    ConnectMQTT();
  }
  mqtt_client.loop();


  if (temp_measurement_freq_ms + last_temp_measurement_ms < now) {
    // NOTE THAT THIS ONLY QUERIES THE FIRST DS18B20 YOU HAVE
    // CONNECTED TO YOUR ESP8266.  IF YOU HAVE MULTIPLE TEMP
    // SENSORS ATTACHED TO YOUR MICROCONTROLLER, YOU SHOULD
    // USE A FOR LOOP TO ITERATE THROUGH ALL OF THEM!
    celsius_reading = DS18B20.getTempC(device_addresses[0]);
    fahrenheit_reading = DS18B20.getTempF(device_addresses[0]);

    // If you're using a low resolution (quicker measurement times),
    // then you can probably set this to false.
    DS18B20.setWaitForConversion(true);

    DS18B20.requestTemperatures();

    last_temp_measurement_ms = millis();
  }

  if (millis() - print_time > PRINT_INTERVAL)
  {
    // publish formatted message to MQTT topic
    snprintf(
      mqtt_message,
      MQTT_MESSAGE_SIZE,
      "{ \"device_id\": \"%s\", \"temp_c\": %.2f, \"temp_f\": %.2f, \"ph\": %.2f, \"ph_mv\": %.2f }",
      DeviceIdToString(device_addresses[0]).c_str(),  // snprintf wants a const char*
      celsius_reading,
      fahrenheit_reading,
      ph_value,
      raw_voltage
    );
    mqtt_client.publish(mqtt_topic, mqtt_message);
    print_time = millis();

    Serial.println(mqtt_message);
  }
}
