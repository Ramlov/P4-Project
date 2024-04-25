use embassy::time::Duration;
use embassy_stm32::gpio::{Input, Level, Pull};
use embassy_stm32::Peripherals;
use embassy_stm32::interrupt;
use embassy_stm32::interrupt::Interrupt;
use embassy_stm32::time::Hertz;
use embassy_stm32::pac::interrupt;
use embassy_executor::task;
use embassy_executor::Executor;
use embassy_stm32::rcc::RccExt;
use embassy_stm32::gpio::NoPin;
use embassy_stm32::gpio::gpioa::PA9;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::pac::EXTI;

// Define a global variable to hold the PA9 pin
static mut PA9_PIN: Option<ExtiInput<'static, PA9>> = None;

// Define an interrupt handler for the timer
#[interrupt]
fn EXTI9_5() {
    // Safety: This interrupt handler assumes that PA9 pin is configured correctly
    unsafe {
        if let Some(ref mut pin) = PA9_PIN {
            if pin.check_interrupt() {
                pin.clear_interrupt_pending_bit();
                if pin.is_high().unwrap() {
                    // PA9 pin is high
                    defmt::info!("PA9 pin is high");
                } else {
                    // PA9 pin is low
                    defmt::info!("PA9 pin is low");
                }
            }
        }
    }
}

#[task]
async fn main_task(spawner: embassy_executor::Spawner, p: embassy_stm32::Peripherals) {
    let mut rcc = p.RCC.freeze(Hertz(16_000_000), p.FLASH.constrain());

    let gpioa = p.GPIOA.split(&mut rcc);

    let pa9 = gpioa.pa9.pull_type(Pull::Up).input();
    let pa9 = embassy_stm32::exti::ExtiInput::new(pa9, p.EXTI);

    // Safety: We know that PA9_PIN is None at this point, so it's safe to write to it
    unsafe {
        PA9_PIN = Some(pa9);
    }

    loop {
        embassy::time::Timer::after(Duration::from_millis(2000)).await;
        EXTI9_5();
    }
}

#[embassy::main]
async fn main(spawner: embassy_executor::Spawner, p: embassy_stm32::Peripherals) {
    spawner.spawn(main_task(spawner, p)).unwrap();
}