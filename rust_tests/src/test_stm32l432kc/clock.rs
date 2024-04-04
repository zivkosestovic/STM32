extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

extern crate stm32l4xx_hal as hal;

use hal::delay::Delay;
use hal::prelude::*;
use stm32l4xx_hal::rcc::Clocks;

pub fn clock_test() {

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    
    let mut flash = dp.FLASH.constrain(); // .constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led = gpioa.pa4.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    system_init();

    let mut rcc_clocks_type_def = RCC_ClocksTypeDef {
        SYSCLK_Frequency    : 0,
        HCLK_Frequency      : 0,
        PCLK1_Frequency     : 0,
        PCLK2_Frequency     : 0,
        ADCCLK_Frequency    : 0,
    };
            
    rcc_get_clocks_frequency(&mut rcc_clocks_type_def);

    let clocks = Clocks {
        hclk                : rcc_clocks_type_def.HCLK_Frequency.Hz(),
        pclk1               : rcc_clocks_type_def.PCLK1_Frequency.Hz(),
        pclk2               : rcc_clocks_type_def.PCLK2_Frequency.Hz(),
        sysclk              : rcc_clocks_type_def.SYSCLK_Frequency.Hz(),
        hsi48               : false,
        msi                 : None,
        lsi                 : false,
        lse                 : false,
        ppre1               : 0,
        ppre2               : 0,
        timclk1             : 0.Hz(),
        timclk2             : 0.Hz(),
        pll_source          : None,
    };

    let mut timer = Delay::new(cp.SYST, clocks);
    loop {
        // block!(timer.wait()).unwrap();
        timer.delay_ms(1000_u32);
        led.set_high();
        // block!(timer.wait()).unwrap();
        timer.delay_ms(1000_u32);
        led.set_low();
    }
}
