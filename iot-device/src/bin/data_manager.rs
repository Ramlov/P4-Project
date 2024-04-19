#![no_std]
#![no_main]

// imports
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, peripherals};
use {defmt_rtt as _, panic_probe as _};


// traits (think of them as interfaces from java)

// The DataManager class
struct DataManager {
    // dummy fields (attributes) - change to define as correct datatypes
    swc: u8,
    temp_data: u8,
    moisture_data: u8,
}

impl DataManager {  // dummy return types - change to define as correct datatypes
    fn get_swc_state() -> () {

    }

    fn confirm_swc_state() -> () {

    }

    fn set_swc() -> () {

    }

    fn append_data() -> () {

    }

    fn clear_data() -> () {

    }

    fn return_data() -> () {

    }

}


#[embassy_executor::main]      // Hopefully only runs if this file is the entrypoint (think python: if __name__ == "__main__")
async fn main(_spawner: Spawner) {
    let swc: u8 = 0;
    let temp_data: u8 = 0;
    let moisture_data: u8 = 0;
    let dm: DataManager = DataManager{
        swc: swc,
        temp_data: temp_data, 
        moisture_data: moisture_data};

    defmt::info!("This is the Data Manager instance: {:?}", dm.swc);
}
