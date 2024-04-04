#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate stm32l1xx_hal as hal;

use core::panic::PanicInfo;
use embedded_hal::digital::v2::OutputPin;
use hal::prelude::*;
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb7.into_push_pull_output();

    loop {
        for _ in 0..1_000 {
            led.set_high().unwrap();
        }
        for _ in 0..1_000 {
            led.set_low().unwrap();
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
