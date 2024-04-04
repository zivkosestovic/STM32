use stm32f4xx_hal as mcu;

use mcu::pac;
use mcu::prelude::*;

//________ System library import _________

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;
use mcu::rcc::Clocks;

pub fn clock_test() {

    system_init(); // Initialization of Clock

    if let (Some(peri), Some(cortex_peri)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // GPIO Initialization
        let gpioa = peri.GPIOA.split();
        let mut red_led = gpioa.pa4.into_push_pull_output();
        
        let mut rcc_clocks_type_def = RCC_ClocksTypeDef
        {
            SYSCLK_Frequency    : 0,
            HCLK_Frequency      : 0,
            PCLK1_Frequency     : 0,
            PCLK2_Frequency     : 0,
        };

        let clocks = rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
        
        let clocks = Clocks
        {
           hclk     : rcc_clocks_type_def.HCLK_Frequency.Hz(),
           pclk1    : rcc_clocks_type_def.PCLK1_Frequency.Hz(),
           pclk2    : rcc_clocks_type_def.PCLK2_Frequency.Hz(),
           sysclk   : rcc_clocks_type_def.SYSCLK_Frequency.Hz(),
           timclk1  : 0.Hz(),
           timclk2  : 0.Hz(),
           pll48clk : None,
           i2s_clk  : None,
           saia_clk : None,
           saib_clk : None,
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