extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

extern crate stm32g0xx_hal as hal;

use stm32g0xx_hal::hal::digital::v2::ToggleableOutputPin;
use hal::prelude::*;
use hal::stm32;
use hal::rcc::Config;
use stm32g0xx_hal::rcc::SysClockSrc;
use stm32g0xx_hal::rcc::*;
pub fn clock_test() {
    
    let dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain().freeze( Config {
        sys_mux: SysClockSrc::HSI(Prescaler::NotDivided),
        pll_cfg: PllConfig::default(),
        ahb_psc: Prescaler::NotDivided,
        apb_psc: Prescaler::NotDivided,
    });

    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output();

    system_init();

    loop {
        led.toggle().unwrap();
        Delay_ms(1000);
    }
}