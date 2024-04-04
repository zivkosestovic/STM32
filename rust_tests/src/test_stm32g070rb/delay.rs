extern crate stm32g0xx_hal as hal;

use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

pub fn delay_test() {
    
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.freeze(Config::default());

    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output();
    
    system_init();

    loop {
        led.toggle().unwrap();
        Delay_ms(1000);
    }
}