#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.freeze(Config::hsi());
    let mut delay = cp.SYST.delay(rcc.clocks);

    let gpioa = dp.GPIOA.split();
    let button = gpioa.pa0.into_pull_up_input();

    let gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb6.into_push_pull_output();

    loop {
        if button.is_high().unwrap() {
            led.set_high().unwrap();
            delay.delay(500.ms());
        } else {
            led.set_low().unwrap();
        }
    }
}
