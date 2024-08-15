#include <WiFi.h>
#include <WebServer.h>

// web server setup
const char* ssid = "Not_Silo";
const char* password = "silo";
IPAddress local_ip(192,168,1,2);
IPAddress gateway(192,168,1,1);
IPAddress subnet(255,255,255,0);
WebServer server(80);

uint8_t LEDpin = 2;
bool LEDstatus = LOW;

void setup() {
  Serial.begin(115200);
  pinMode(LEDpin, OUTPUT);

  // Set up the Wi-Fi Access Point
  WiFi.softAP(ssid, password);
  WiFi.softAPConfig(local_ip, gateway, subnet);

  // Wait for the network to initialize
  delay(100);
`
  // Define server routes
  server.on("/", handle_OnConnect);
  server.on("/ledon", handle_ledon);
  server.on("/ledoff", handle_ledoff);
  server.onNotFound(handle_NotFound);

  // Start the server
  server.begin();
  Serial.println("HTTP server started");
}

void loop() {
  server.handleClient();

  // Control LEDs based on their status
  digitalWrite(LEDpin, LEDstatus ? HIGH : LOW);
}

void handle_OnConnect() {
  LEDstatus = LOW;
  Serial.println("BOOT: LED OFF");
}

void handle_ledon() {
  LEDstatus = HIGH;
  Serial.println("LED: ON");
}

void handle_ledoff() {
  LEDstatus = LOW;
  Serial.println("LED OFF");
}

void handle_NotFound(){
  server.send(404, "text/plain", "Not found");
}

String SendHTML(bool ledstat){
  String ptr = "<!DOCTYPE html> <html>\n";
  ptr +="<head><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, user-scalable=no\">\n";
  ptr +="<meta http-equiv=\"refresh\" content=\"0;url=http://192.168.1.2\">\n";  // Redirect to 192.168.1.2
  ptr +="<title>LED Control</title>\n";
  ptr +="<style>html { font-family: Helvetica; display: inline-block; margin: 0px auto; text-align: center;}\n";
  ptr +="body{margin-top: 50px;} h1 {color: #444444;margin: 50px auto 30px;} h3 {color: #444444;margin-bottom: 50px;}\n";
  ptr +=".button {display: block;width: 80px;background-color: #3498db;border: none;color: white;padding: 13px 30px;text-decoration: none;font-size: 25px;margin: 0px auto 35px;cursor: pointer;border-radius: 4px;}\n";
  ptr +=".button-on {background-color: #3498db;}\n";
  ptr +=".button-on:active {background-color: #2980b9;}\n";
  ptr +=".button-off {background-color: #34495e;}\n";
  ptr +=".button-off:active {background-color: #2c3e50;}\n";
  ptr +="p {font-size: 14px;color: #888;margin-bottom: 10px;}\n";
  ptr +="</style>\n";
  ptr +="</head>\n";
  ptr +="<body>\n";
  ptr +="<h1>ESP32 Web Server</h1>\n";
  ptr +="<h3>Using Access Point(AP) Mode</h3>\n";
  
  if(ledstat)
    ptr +="<p>LED Status: OFF</p><a class=\"button button-on\" href=\"/ledon\">ON</a>\n";
  else
    ptr +="<p>LED Status: ON</p><a class=\"button button-off\" href=\"/ledoff\">OFF</a>\n";

  ptr +="</body>\n";
  ptr +="</html>\n";
  return ptr;
}
