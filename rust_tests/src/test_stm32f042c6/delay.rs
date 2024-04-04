extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use panic_halt as _;

use stm32f0xx_hal::{pac, prelude::*, rcc::Clocks};
use cortex_m::peripheral::Peripherals;

pub fn delay_test() {

    system_init(); // Initialization of Clock
    let mut dp = pac::Peripherals::take().unwrap();

    let mut rcc_clocks_type_def = RCC_ClocksTypeDef
    {
        SYSCLK_Frequency    : 0,
        HCLK_Frequency      : 0,
        PCLK_Frequency      : 0,
        ADCCLK_Frequency    : 0,
    };
            
    rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
            
    let clocks = Clocks
    {
        hclk     : rcc_clocks_type_def.HCLK_Frequency.hz(),
        pclk     : rcc_clocks_type_def.PCLK_Frequency.hz(),
        sysclk   : rcc_clocks_type_def.SYSCLK_Frequency.hz(),
    };
    

    let mut rcc = dp.RCC.configure()
    .sysclk(clocks.sysclk)  // Set the desired system clock frequency
    .hclk(clocks.hclk)    // Set the desired AHB bus frequency
    .pclk(clocks.pclk)    // Set the desired APB bus frequency
    .freeze(&mut dp.FLASH);

    let gpioa = dp.GPIOA.split(&mut rcc);
        

    // (Re-)configure PA1 as output
    let mut led = cortex_m::interrupt::free(move |cs| gpioa.pa4.into_push_pull_output(cs));
     
    loop {
        led.toggle();
        Delay_ms(1000);
    }
    

    loop {
        continue;
    }
}
