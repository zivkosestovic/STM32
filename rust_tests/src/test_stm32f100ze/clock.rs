//! Demonstrate the use of a blocking `Delay` using TIM2 general-purpose timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]

// Halt on panic
use panic_halt as _; // panic handler

use stm32f1xx_hal::{pac, prelude::*, rcc::Clocks};
use cortex_m::peripheral::Peripherals;
//________ System library import _________

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

pub fn clock_test() {
    system_init();
    if let (Some(dp), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {

    let mut rcc_clocks_type_def = RCC_ClocksTypeDef {
        SYSCLK_Frequency    : 0,
        HCLK_Frequency      : 0,
        PCLK1_Frequency     : 0,
        PCLK2_Frequency     : 0,
        ADCCLK_Frequency    : 0,
    };
            
    rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
            
    let clocks = Clocks {
        hclk: rcc_clocks_type_def.HCLK_Frequency.Hz(),
        pclk1: rcc_clocks_type_def.PCLK1_Frequency.Hz(),
        pclk2: rcc_clocks_type_def.PCLK2_Frequency.Hz(),
        ppre1: 0,
        ppre2: 0,
        sysclk: rcc_clocks_type_def.SYSCLK_Frequency.Hz(),
        adcclk: rcc_clocks_type_def.ADCCLK_Frequency.Hz(),
    };

    let mut gpioa = dp.GPIOA.split();

    let mut led = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);

    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.toggle();
        delay.delay_ms(1_000_u16);
    }
}

loop {
    continue;
}
}
