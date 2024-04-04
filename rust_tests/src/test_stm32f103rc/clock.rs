//! Demonstrate the use of a blocking `Delay` using TIM2 general-purpose timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]

// Halt on panic
use panic_halt as _; // panic handler

use stm32f1xx_hal::{pac, prelude::*,  rcc::Clocks};

//________ System library import _________

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

pub fn clock_test() {

    system_init(); 

    if let (Some(dp), Some(_cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let mut flash = dp.FLASH.constrain();
        // Set up the LED. On the BluePill it's connected to pin PC13.
        let mut gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);

            
        let mut rcc_clocks_type_def = RCC_ClocksTypeDef
        {
            SYSCLK_Frequency    : 0,
            HCLK_Frequency      : 0,
            PCLK1_Frequency     : 0,
            PCLK2_Frequency     : 0,
            ADCCLK_Frequency    : 0,
        };


        let clocks = rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
 
        let clocks = Clocks
        {
            hclk: rcc_clocks_type_def.HCLK_Frequency.Hz(),
            pclk1: rcc_clocks_type_def.PCLK1_Frequency.Hz(),
            pclk2: rcc_clocks_type_def.PCLK2_Frequency.Hz(),
            ppre1: 0,
            ppre2: 0,
            sysclk: rcc_clocks_type_def.SYSCLK_Frequency.Hz(),
            adcclk: rcc_clocks_type_def.ADCCLK_Frequency.Hz(),
            usbclk_valid: false,
        };

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc
            .cfgr
            .sysclk(clocks.sysclk)
            .hclk(clocks.hclk)
            .pclk1(clocks.pclk1)
            .pclk2(clocks.pclk2)
            .freeze(&mut flash.acr);

        // Create a delay abstraction based on general-pupose 32-bit timer TIM2

        //let mut delay = hal::timer::FTimerUs::new(dp.TIM2, &clocks).delay();
        // or
        let mut delay = dp.TIM2.delay_us(&clocks);

        loop {
            led.set_high();
            delay.delay_ms(1000_u32);
            led.set_low();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}
