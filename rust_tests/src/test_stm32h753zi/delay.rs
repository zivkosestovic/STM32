
extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use stm32h7xx_hal::prelude::*;
pub fn delay_test() {
  
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();
    
    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOB peripheral
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);

    // Configure gpio B pin 0 as a push-pull output.
    let mut led = gpioa.pa4.into_push_pull_output();

    system_init();

    loop {
        led.toggle();
        Delay_ms(1000);
    }
}
