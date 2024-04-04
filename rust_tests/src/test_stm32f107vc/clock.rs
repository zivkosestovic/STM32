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
        // Set up the LED. On the BluePill it's connected to pin PC13.
        let mut gpiob = dp.GPIOB.split();
        let mut led = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);

            
        let mut rcc_clocks_type_def = RCC_ClocksTypeDef
        {
            SYSCLK_Frequency    : 0,
            HCLK_Frequency      : 0,
            PCLK1_Frequency     : 0,
            PCLK2_Frequency     : 0,
            ADCCLK_Frequency    : 0,
        };


        rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
 
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

        let mut delay = dp.TIM2.delay_us(&clocks);

        loop {
            led.toggle();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}
