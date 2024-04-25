#include <Arduino.h>
#include <OneWire.h>
#include <DallasTemperature.h>
#include "sensor.h"

// Define sensor module object
SensorModule sensorModule(A1, 500, 200, 17); // A1: Moisture sensor pin, 500: Dry reading, 200: Wet reading, 17: DS18B20 digital data pin

void setup() {
  Serial.begin(9600);
}

void loop() {
  int moisturePercentage = sensorModule.getMoisture();
  float temperature = sensorModule.getTemperature();

  Serial.print("Moisture Percentage: ");
  Serial.print(moisturePercentage);
  Serial.println("%");

  Serial.print("Temperature: ");
  Serial.print(temperature);
  Serial.println(" °C");

  delay(1000);
}
