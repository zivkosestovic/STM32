#![deny(unsafe_code)]
#![deny(warnings)]
extern crate panic_halt;

use stm32f7xx_hal as hal;

use hal::{pac, prelude::*};

pub fn gpio_test() {
    let p = pac::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let mut led = gpioa.pa4.into_push_pull_output();

    loop {
        for _ in 0..100_000 {
            led.set_high();
        }
        for _ in 0..100_000 {
            led.set_low();
        }
    }
}
