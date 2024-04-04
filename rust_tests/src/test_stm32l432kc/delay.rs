extern crate stm32l4xx_hal as hal;

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use hal::prelude::*;

pub fn delay_test() {

    let dp = hal::stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led = gpioa.pa4.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    system_init();

    loop {
        led.toggle();
        Delay_ms(1000);
    }
}
