#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi());
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    let mut adc_pin = gpioa.pa1.into_analog();
    let mut adc = dp.ADC.adc(&mut rcc);

    let mut pwm = dp.TIM4.pwm(gpiob.pb7, 1.khz(), &mut rcc);
    let max_duty = pwm.get_max_duty() / 4095;
    pwm.enable();

    loop {
        let val: u16 = adc.read(&mut adc_pin).unwrap();
        pwm.set_duty(max_duty * val);
    }
}
