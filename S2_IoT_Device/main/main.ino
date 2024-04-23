/*
Main point of execution for the S2: IoT Device
*/


// Define the TEST_MODE flag to enable/disable test code
#define TEST_MODE


// include libraries
#include <Arduino.h>

// include classes
#include "data_manager.h"
#include "sensor_manager.h"
#include "lorawan_manager.h"
#include "task_orchestrator.h"

// include unit tests
#include "tests/test_data_manager.cpp"
#include "tests/test_sensor_manager.cpp"
#include "tests/test_lorawan_manager.cpp"
#include "tests/test_task_orchestrator.cpp"





void setup() {
  // Initialize hardware and other setup routines

    #ifdef TEST_MODE
    // Run test code when TEST_MODE is defined
    run_tests();
  #endif
}

void loop() {
  // Main program logic
  
  // Other loop logic
}


// Test function to run unit tests
void run_tests() {
  // Instantiate test objects
  
  // Run tests for each component
}