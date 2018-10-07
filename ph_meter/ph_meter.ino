// SEN 0169 reader and LCD display

#include <LiquidCrystal.h>

#define printInterval 800
#define LED 13
#define SensorPin A0            //pH meter Analog output to Arduino Analog Input 0

#define samplingInterval 20
#define ArrayLength  40          //times of collection -- we raised this MUCH higher than mfr recc
int pHArray[ArrayLength];        //Store the average value of the sensor feedback
int pHArrayIndex=0;

// LCD pins in order: RS, EN, D4, D5, D6, D7
LiquidCrystal lcd(2, 3, 8, 9, 10, 11); 

void setup(void)
{
  // set number of columns and rows for the LCD
  lcd.begin(16, 2);
  pinMode(13,OUTPUT);  
  
}
void loop(void)
{
  static unsigned long samplingTime = millis();
  static unsigned long printTime = millis();
  static float pHValue, raw_voltage;
  
  static float blue_mv = 399.66;  // millivolt reading at  7.0  ph reference solution
  static float red_mv = 330.76;   // millivolt reading at  4.01 ph reference solution
  static float blue_ph_minus_red_ph = 3.0;  // 7.0 - 4.0 (diff btw testing solutions)
  
  if(millis()-samplingTime > samplingInterval)
  {
      pHArray[pHArrayIndex++]=analogRead(SensorPin);
      if(pHArrayIndex==ArrayLength)pHArrayIndex=0;
      raw_voltage = (array_avg(pHArray, ArrayLength));
      float i = raw_voltage - blue_mv;
      pHValue = (i / ((blue_mv - red_mv) / blue_ph_minus_red_ph)) + 7.0;
      samplingTime=millis();
  }
  if(millis() - printTime > printInterval)   
  {
    
    digitalWrite(LED,digitalRead(LED)^1);
    
    lcd.print("mV ");
    lcd.print(raw_voltage,2);
    lcd.setCursor(0, 1); // line 2
    lcd.print("pH value ");
    lcd.print(pHValue,2);
    lcd.setCursor(0, 0);
    printTime=millis();
  }

}
double array_avg(int* arr, int number){
  int i;
  int max,min;
  double avg;
  long amount=0;
  if(number<=0){
    Serial.println("Error number for the array to avraging!/n");
    return 0;
  }
  if(number<5){   //less than 5, calculated directly statistics
    for(i=0;i<number;i++){
      amount+=arr[i];
    }
    avg = amount/number;
    return avg;
  }else{
    if(arr[0]<arr[1]){
      min = arr[0];max=arr[1];
    }
    else{
      min=arr[1];max=arr[0];
    }
    for(i=2;i<number;i++){
      if(arr[i]<min){
        amount+=min;        //arr<min
        min=arr[i];
      }else {
        if(arr[i]>max){
          amount+=max;    //arr>max
          max=arr[i];
        }else{
          amount+=arr[i]; //min<=arr<=max
        }
      }//if
    }//for
    avg = (double)amount/(number-2);
  }//if
  return avg;
}
