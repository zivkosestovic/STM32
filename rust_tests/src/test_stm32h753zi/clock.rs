
use stm32h7xx_hal::{pac, prelude::*};

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use stm32h7xx_hal::rcc::CoreClocks;
use stm32h7xx_hal::delay::Delay;
pub fn clock_test() {

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let pwrcfg = (pwr).freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);

    let mut led = gpioa.pa4.into_push_pull_output();
    
    system_init();

    loop {
        led.toggle();
        Delay_ms(1000_u32);
    }
}