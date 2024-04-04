extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use stm32l0xx_hal::{pac, prelude::*};

pub fn delay_test() {

    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(stm32l0xx_hal::rcc::Config::hsi16());

    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output();

    system_init();

    loop {
        led.toggle().unwrap();
        Delay_ms(1000);
    }
}
