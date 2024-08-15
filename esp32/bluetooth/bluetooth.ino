#include "BluetoothSerial.h"

const int LED = 2;              
const char* turnON = "on";       
const char* turnOFF = "off";

#if !defined(CONFIG_BT_ENABLED) || !defined(CONFIG_BLUEDROID_ENABLED)
#error Bluetooth is not enabled! Please run `make menuconfig` to and enable it
#endif

BluetoothSerial SerialBT;
String msg = "";  

void setup() {
  Serial.begin(115200);
  pinMode(LED, OUTPUT);
  SerialBT.begin("ESP32test"); 
  Serial.println("The device started, now you can pair it with Bluetooth!");
  Serial.println("Now You can TURN ON LED by sending 'on' and TURN OFF by 'off'");
}

void loop() {
  if (SerialBT.available()) {
    char incomingChar = SerialBT.read();
    
    if (incomingChar != '\n' && incomingChar != '\r') {
      msg += incomingChar;
    } else {
      if (msg.length() > 0) {  
        Serial.println("Received message: " + msg);
        
        if (msg.equals(turnON)) {  
          digitalWrite(LED, HIGH);
          Serial.println("LED Turned ON");
          SerialBT.println("LED Turned ON");
        } 
        else if (msg.equals(turnOFF)) {
          digitalWrite(LED, LOW);
          Serial.println("LED Turned OFF");
          SerialBT.println("LED Turned OFF");
        } 
        else {
          Serial.println("Invalid Input");
          SerialBT.println("Invalid Input");
        }
        
        msg = "";  // Clear the buffer for the next message
      }
    }
  }

  delay(20);
}
