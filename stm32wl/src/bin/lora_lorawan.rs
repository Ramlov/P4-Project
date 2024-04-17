//! This example runs on a STM32WL board, which has a builtin Semtech Sx1262 radio.
//! It demonstrates LoRaWAN join functionality.
#![no_std]
#![no_main]

#[path = "../iv.rs"]
mod iv;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_stm32::rng::{self, Rng};
use embassy_stm32::spi::Spi;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_time::Delay;
use lora_phy::lorawan_radio::LorawanRadio;
use lora_phy::sx126x::{self, Sx126x, Sx126xVariant, TcxoCtrlVoltage};
use lora_phy::LoRa;
use lorawan_device::async_device::{region, Device, EmbassyTimer, JoinMode, JoinResponse, SendResponse};
use lorawan_device::default_crypto::DefaultFactory as Crypto;
use lorawan_device::{AppEui, AppKey, DevEui};
use {defmt_rtt as _, panic_probe as _};

use self::iv::{InterruptHandler, Stm32wlInterfaceVariant, SubghzSpiDevice};

// warning: set these appropriately for the region
const LORAWAN_REGION: region::Region = region::Region::EU868;
const MAX_TX_POWER: u8 = 22;

bind_interrupts!(struct Irqs{
    SUBGHZ_RADIO => InterruptHandler;
    RNG => rng::InterruptHandler<peripherals::RNG>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(32_000_000),
            mode: HseMode::Bypass,
            prescaler: HsePrescaler::DIV1,
        });
        config.rcc.mux = ClockSrc::PLL1_R;
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL6,
            divp: None,
            divq: Some(PllQDiv::DIV2), // PLL1_Q clock (32 / 2 * 6 / 2), used for RNG
            divr: Some(PllRDiv::DIV2), // sysclk 48Mhz clock (32 / 2 * 6 / 2)
        });
    }
    let p = embassy_stm32::init(config);

    // Set CTRL1 and CTRL3 for high-power transmission, while CTRL2 acts as an RF switch between tx and rx
    let _ctrl1 = Output::new(p.PC4.degrade(), Level::Low, Speed::High);
    let ctrl2 = Output::new(p.PC5.degrade(), Level::High, Speed::High);
    let _ctrl3 = Output::new(p.PC3.degrade(), Level::High, Speed::High);

    let spi = Spi::new_subghz(p.SUBGHZSPI, p.DMA1_CH1, p.DMA1_CH2);
    let spi = SubghzSpiDevice(spi);

    let config = sx126x::Config {
        chip: Sx126xVariant::Stm32wl,
        tcxo_ctrl: Some(TcxoCtrlVoltage::Ctrl1V7),
        use_dcdc: true,
        use_dio2_as_rfswitch: true,
        rx_boost: false,
    };
    let iv = Stm32wlInterfaceVariant::new(Irqs, None, Some(ctrl2)).unwrap();
    let lora = LoRa::new(Sx126x::new(spi, iv, config), true, Delay).await.unwrap();

    let radio: LorawanRadio<_, _, MAX_TX_POWER> = lora.into();
    let region: region::Configuration = region::Configuration::new(LORAWAN_REGION);
    let mut device: Device<_, Crypto, _, _> = Device::new(region, radio, EmbassyTimer::new(), Rng::new(p.RNG, Irqs));

    defmt::info!("Joining LoRaWAN network");

    // TODO: Adjust the EUI and Keys according to your network credentials
    let resp = device
        .join(&JoinMode::OTAA {
            deveui: DevEui::from([0xFA, 0x2B, 0x60, 0x52, 0x20, 0xF1, 0xF7, 0x2C]), //LSB
            appeui: AppEui::from([0xE4, 0x15, 0x00, 0x00, 0x00, 0x00, 0x7A, 0xBE]), //LSB
            appkey: AppKey::from([0x66,0x86,0x05,0x16,0x96,0x0B,0x9D,0xDE,0xE5,0x4C,0x48,0xDA,0xD6,0x88,0x4F,0xC7]),    //MSB
        })
        .await
        .unwrap();

    info!("LoRaWAN network joined: {:?}", resp);

        // loop {
            info!("Sending uplink...");
            let result = device.send(&[0x01, 0x02, 0x03, 0x04], 1, true).await;
            if let Ok(SendResponse::DownlinkReceived(_)) = result {
                // After an uplink with Class C enabled, it is important to check for multiple downlinks.
                // It is theoretically possible to receive a Class A downlink and any number of Class C
                // downlinks during the Class C windows.
                while let Some(downlink) = device.take_downlink() {
                    info!("Received {:?}", downlink);
                }
                // break;
            } else {
                info!("Uplink failed: {:?}. Retrying...", result);
            }
        // }

    // let mut data:[u8; 5] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    // device.send(&data, 1, false).await.unwrap();
}
