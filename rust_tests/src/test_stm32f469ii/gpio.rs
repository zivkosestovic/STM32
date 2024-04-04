#![deny(unsafe_code)]
#![deny(warnings)]

use panic_halt as _;

use stm32f4xx_hal::{pac, prelude::*};

pub fn gpio_test() {
    let p = pac::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let mut led = gpioa.pa4.into_push_pull_output();

    loop {
        for _ in 0..10_000 {
            led.set_high();
        }
        for _ in 0..10_000 {
            led.set_low();
        }
    }
}
