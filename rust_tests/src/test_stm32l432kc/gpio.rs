
extern crate stm32l4xx_hal as hal;

use hal::prelude::*;

pub fn gpio_test() {
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led = gpioa.pa4.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    loop {
        led.set_high();
    }
}
