# P4-Project

## S1: IoT Device
The code for the IoT Device can be found in the 'iot-device' folder.

You should download the 'lora-rs' repository and put it in /iot-device/, so that you end up with the path to it being
/iot-device/lora-rs/. The path is important for the project to work properly.

Do not alter the 'lora-rs' folder! It contains all LoRa and LoRaWAN related crates and setup configurations. The repo for 'lora-rs' can be found here (date installed 17/4 - 2024): https://github.com/lora-rs/lora-rs

Our Wio E5 mini is a build around a STM32WLE5JCixx. Alter the stm32wle5jc to your chip name in /iot-device/Cargo.toml, if necessary. Also update /iot-device/.cargo/config.toml 


## Useful Cargo commands:
Make sure to call these with the working directory being /iot-device/ in order for cargo to be able to find the 'Cargo.toml' file.

When uploading the main code to the wio E5 mini:

> cargo run -r --bin main

When checking if all files in /src/bin/ will compile:

> cargo check

When checking single file in /src/bin/:

> cargo check --bin filename        (excluding .rs)

When compiling and building file:

> cargo build --bin filename        (add -r flag if building for 'release'. Will compress and optimize)

Using 'cargo run' will build and then upload the code.


### Device 1 credentials
deveui: DevEui::from([0xFA, 0x2B, 0x60, 0x52, 0x20, 0xF1, 0xF7, 0x2C]), // LSB

appeui: AppEui::from([0xE4, 0x15, 0x00, 0x00, 0x00, 0x00, 0x7A, 0xBE]), // LSB

appkey: AppKey::from([0x66,0x86,0x05,0x16,0x96,0x0B,0x9D,0xDE,0xE5,0x4C,0x48,0xDA,0xD6,0x88,0x4F,0xC7]), // MSB


### Device 2 credentials
deveui: DevEui::from([0xF1, 0x2B, 0x60, 0x52, 0x20, 0xF1, 0xF7, 0x2C]), // LSB

appeui: AppEui::from([0xE4, 0x15, 0x00, 0x00, 0x00, 0x00, 0x7A, 0xBE]), // LSB

appkey: AppKey::from([0x57,0xB3,0x68,0xC5,0xD5,0x5D,0x9F,0xC5,0xDE,0xE1,0x42,0x78,0xB4,0x3F,0x4C,0x52]), // MSB
