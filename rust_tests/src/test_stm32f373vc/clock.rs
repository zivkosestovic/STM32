use panic_halt as _;

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use stm32f3xx_hal::{delay::Delay, pac, prelude::*, rcc::Clocks};

use cortex_m::peripheral::Peripherals;

pub fn clock_test() {
    system_init();
    if let (Some(mut p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {

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
            usbclk_valid: false,
            pll_bypass: false,
        };

       // Configure the RCC to use the desired clock configuration
       let mut rcc = p.RCC.constrain();

       let mut gpioa = p.GPIOA.split(&mut rcc.ahb);

       let mut delay = Delay::new(cp.SYST, clocks);

        let mut led = gpioa.pa4.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

        loop {
            led.toggle().ok();
            delay.delay_ms(1_000_u16);
        }
    }

    loop {
        continue;
    }
}
