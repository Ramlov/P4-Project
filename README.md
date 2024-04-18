# P4-Project

## S1: IoT Device
The code for the IoT Device can be found in the 'iot-device' folder. Do not alter the 'lora-rs' folder! It contains all LoRa and LoRaWAN related crates and setup configurations. The repo for 'lora-rs' can be found here (date installed 17/4 - 2024): https://github.com/lora-rs/lora-rs

Our Wio E5 mini is a build around a STM32WLE5JCixx. Alter the stm32wle5jc to your chip name in /iot-device/Cargo.toml, if necessary. Also update /iot-device/.cargo/config.toml 
