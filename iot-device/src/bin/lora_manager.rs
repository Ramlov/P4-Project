
#![no_std]
#![no_main]

// imports
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, peripherals};
use {defmt_rtt as _, panic_probe as _};


// traits (think of them as interfaces from java)


// The LoRaManager class
struct LoRaManager {
    // dummy fields (attributes) - change to define as correct datatypes
    LoRa_config: u8
}

impl LoRaManager {  // dummy return types - change to define as correct datatypes
    fn set_config() -> () {

    }

}


#[embassy_executor::main]      // Only runs if this file is the entrypoint (think python: if __name__ == "__main__")
async fn main(_spawner: Spawner) {
    let lora_config: u8 = 0;
    let lm: LoRaManager = LoRaManager{
        LoRa_config: lora_config};

    defmt::info!("If this prints, all should be good");

}
