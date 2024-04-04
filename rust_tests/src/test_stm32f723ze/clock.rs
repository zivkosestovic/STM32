use stm32f7xx_hal::pac;
use stm32f7xx_hal::prelude::*;
use stm32f7xx_hal::rcc::Clocks;

//________ System library import _________
extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

pub fn clock_test() {

    system_init(); // Initialization of Clock
    
    if let (Some(peri), Some(mut cortex_peri)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // GPIO Initialization
        let gpioa = peri.GPIOA.split();
        let mut red_led = gpioa.pa4.into_push_pull_output();

        let mut rcc_clocks_type_def = RCC_ClocksTypeDef
        {
            sysclk_frequency : 0,
            hclk_frequency   : 0,
            pclk1_frequency  : 0,
            pclk2_frequency  : 0,
        };

        let clocks = rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
        
        let clocks = Clocks
        {
           hclk           : rcc_clocks_type_def.hclk_frequency.Hz(),
           pclk1          : rcc_clocks_type_def.pclk1_frequency.Hz(),
           pclk2          : rcc_clocks_type_def.pclk2_frequency.Hz(),
           sysclk         : rcc_clocks_type_def.sysclk_frequency.Hz(),
           timclk1        : 0.Hz(),
           timclk2        : 0.Hz(),
           pll48clk_valid : false,
           hse            : None,
           lse            : None,
           lsi            : None,
       };

        let mut delay = cortex_peri.SYST.delay(&clocks);

        loop {
            red_led.set_high();
            delay.delay_ms(1000_u32);
            red_led.set_low();
            delay.delay_ms(1000_u32);
        }
    }
    loop {}
}