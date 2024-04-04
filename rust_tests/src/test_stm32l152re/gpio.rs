use panic_halt as _;

use stm32l1xx_hal as hal;

use hal:: prelude::*;
use hal::stm32;
use stm32l1xx_hal::hal::digital::v2::OutputPin;

pub fn gpio_test() {
    let dp = stm32::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa4.into_push_pull_output();

        loop {
            // Turn PA1 on a million times in a row
            for _ in 0..1_000_000 {
                led.set_high().ok();
            }
            // Then turn PA1 off a million times in a row
            for _ in 0..1_000_000 {
                led.set_low().ok();
            }
        }
}
