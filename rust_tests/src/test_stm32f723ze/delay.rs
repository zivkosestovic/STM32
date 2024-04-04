extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;
use stm32f7xx_hal::{pac, prelude::*};

pub fn delay_test() {

    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa4.into_push_pull_output();
    
    system_init(); // Initialization of Clock
    
    loop {
        // On for 1s, off for 1s.
        led.toggle();
        delay_ms(1000);
    }
}
