#ifndef SENSOR_H
#define SENSOR_H

#include <Arduino.h>
#include <OneWire.h>
#include <DallasTemperature.h>

class SensorModule {
private:
  int moisturePin;
  int dryReading;
  int wetReading;
  OneWire oneWire;
  DallasTemperature tempSensor;

public:
  SensorModule(int moisturePin, int dry, int wet, int tempPin);
  int getMoisture();
  float getTemperature();
};

#endif
