extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

extern crate stm32l1xx_hal as hal;

use stm32l1xx_hal::hal::digital::v2::ToggleableOutputPin;
use hal::prelude::*;
use hal::stm32;
use stm32l1xx_hal::rcc::Clocks;
pub fn clock_test() {
    system_init();
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc_clocks_type_def = RCC_ClocksTypeDef {
        SYSCLK_Frequency    : 0,
        HCLK_Frequency      : 0,
        PCLK1_Frequency     : 0,
        PCLK2_Frequency     : 0,
        ADCCLK_Frequency    : 0,
    };
            
    rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
            
    let clocks = Clocks {
        ahb_clk      : rcc_clocks_type_def.HCLK_Frequency.hz(),
        apb1_clk     : rcc_clocks_type_def.PCLK1_Frequency.hz(),
        apb2_clk     : rcc_clocks_type_def.PCLK2_Frequency.hz(),
        sys_clk      : rcc_clocks_type_def.SYSCLK_Frequency.hz(),
        apb1_tim_clk : 0.hz(),
        apb2_tim_clk : 0.hz(),
    };

    let mut delay = cp.SYST.delay(clocks);

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa4.into_push_pull_output();

    loop {
        led.toggle().unwrap();
        delay.delay_ms(1000_u16);
    }
}