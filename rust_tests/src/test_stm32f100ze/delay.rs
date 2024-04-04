#![deny(unsafe_code)]
use panic_halt as _;

use stm32f1xx_hal::{pac, prelude::*};

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

pub fn delay_test() {


    let dp = pac::Peripherals::take().unwrap();

    let mut gpioa = dp.GPIOA.split();

    let mut led = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);

    system_init();
    
    loop {
        led.toggle();
        Delay_ms(1000);
    }
}