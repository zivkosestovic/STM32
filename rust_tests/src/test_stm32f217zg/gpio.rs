//! Uses the StatefulOutputPin embedded_hal trait to toggle the pin
//! On the stm32 discovery board this is the "south" led
//! Target board: STM32F3DISCOVERY

#![deny(unsafe_code)]

use stm32f2xx_hal as hal;

use hal::pac;
use hal::prelude::*;

pub fn gpio_test() {
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa4.into_push_pull_output();

    loop {
        led.set_high();
    }
}
