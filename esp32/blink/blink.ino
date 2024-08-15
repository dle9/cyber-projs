#define ledPin 2

void setup()
{
  pinMode(ledPin, OUTPUT);

  digitalWrite(ledPin, HIGH);
  delay(5000);
}

void loop()
{
  digitalWrite(ledPin, HIGH);
  delay(1000);
 
  digitalWrite(ledPin, LOW);
  delay(1000);
}