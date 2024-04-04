#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]

use panic_halt as _; // panic handler

use stm32f2xx_hal as hal;

use hal::{pac, prelude::*};

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

pub fn delay_test() {

    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa4.into_push_pull_output();

    system_init();
    
    loop {
        led.toggle();
        Delay_ms(1000);
    }
}
