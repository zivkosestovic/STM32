extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use stm32l0xx_hal::{delay::Delay, pac, prelude::*, rcc::Clocks, rcc::Config};
use cortex_m::peripheral::Peripherals;

use stm32l0xx_hal::rcc::*;

pub fn clock_test() {
    
    // Get access to the device peripherals
    let cp = Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(Config::hsi16());
    // Get the GPIOA peripheral
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure PA4 as push-pull output
    let mut led = gpioa.pa4.into_push_pull_output();

    // Initialize the system clock
    system_init();


    // Get the clock frequencies from system_init
    let mut rcc_clocks_type_def = RCC_ClocksTypeDef {
        SYSCLK_Frequency    : 0,
        HCLK_Frequency      : 0,
        PCLK1_Frequency     : 0,
        PCLK2_Frequency     : 0,
        ADCCLK_Frequency    : 0,
    };
                
    rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
    let clock_source = ClockSrc::HSI16(HSI16Div::Div1);
    
    // Configure the RCC peripheral with the Clocks struct obtained from system_init
    let clocks = Clocks {
        source       :     clock_source,
        ahb_clk      : rcc_clocks_type_def.HCLK_Frequency.Hz(),
        apb1_clk     : rcc_clocks_type_def.PCLK1_Frequency.Hz(),
        apb2_clk     : rcc_clocks_type_def.PCLK2_Frequency.Hz(),
        sys_clk      : rcc_clocks_type_def.SYSCLK_Frequency.Hz(),
        apb2_tim_clk : 0.Hz(),
        apb1_tim_clk : 0.Hz(),
    };

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high().unwrap();  // Turn LED on
        delay.delay_ms(1000_u32);     // Delay 1 second
        led.set_low().unwrap();   // Turn LED off
        delay.delay_ms(1000_u32);     // Delay 1 second
    }
}