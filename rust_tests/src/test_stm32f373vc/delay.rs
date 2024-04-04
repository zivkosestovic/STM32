extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use stm32f3xx_hal::{
    delay::Delay,
    pac,
    prelude::*,
};

pub fn delay_test() {
    // Get access to the device peripherals
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();

    // Get access to GPIOA
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    // Set up the LED. On the Nucleo-446RE it's connected to pin PA4.
    let mut led = gpioa.pa4.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    system_init();

    loop {
        // On for 1s, off for 1s.
        led.toggle().unwrap(); // Toggle the LED
        Delay_ms(1000);
    }
}