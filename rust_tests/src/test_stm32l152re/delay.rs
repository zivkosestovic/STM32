extern crate stm32l1xx_hal as hal;

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use stm32l1xx_hal::hal::digital::v2::ToggleableOutputPin;
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;

pub fn delay_test() {
    
    let dp = stm32::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa4.into_push_pull_output();

    system_init();
    loop {
        led.toggle().unwrap();
        Delay_ms(1000);
    }
}
