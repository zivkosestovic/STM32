use panic_halt as _;

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use stm32f2xx_hal::{delay::Delay, pac, prelude::*, rcc::Clocks};

use cortex_m::peripheral::Peripherals;

pub fn clock_test() {
    system_init();
    if let (Some(mut p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {

        let mut rcc_clocks_type_def = RCC_ClocksTypeDef {
            SYSCLK_Frequency    : 0,
            HCLK_Frequency      : 0,
            PCLK1_Frequency     : 0,
            PCLK2_Frequency     : 0,
        };
                
        rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
                
        let clocks = Clocks {
            hclk: rcc_clocks_type_def.HCLK_Frequency.hz(),
            pclk1: rcc_clocks_type_def.PCLK1_Frequency.hz(),
            pclk2: rcc_clocks_type_def.PCLK2_Frequency.hz(),
            ppre1: 0,
            ppre2: 0,
            sysclk: rcc_clocks_type_def.SYSCLK_Frequency.hz(),
            pll48clk: None,
        };

       let gpioa = p.GPIOA.split();
       let mut led = gpioa.pa4.into_push_pull_output();

       let mut delay = Delay::new(cp.SYST, &clocks);

        loop {
            led.toggle();
            delay.delay_ms(1_000_u16);
        }
    }

    loop {
        continue;
    }
}
