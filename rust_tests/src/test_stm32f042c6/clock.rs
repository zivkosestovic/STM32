use panic_halt as _;

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use stm32f0xx_hal::{delay::Delay, pac, prelude::*, rcc::Clocks};

use cortex_m::peripheral::Peripherals;

pub fn clock_test() {
    system_init();
    if let (Some(mut p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {

        let mut rcc_clocks_type_def = RCC_ClocksTypeDef {
            SYSCLK_Frequency    : 0,
            HCLK_Frequency      : 0,
            PCLK_Frequency      : 0,
            ADCCLK_Frequency    : 0,
        };
                
        rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
                
        let clocks = Clocks {
            hclk     : rcc_clocks_type_def.HCLK_Frequency.hz(),
            pclk     : rcc_clocks_type_def.PCLK_Frequency.hz(),
            sysclk   : rcc_clocks_type_def.SYSCLK_Frequency.hz(),
        };

        let mut rcc = p.RCC.configure().sysclk(clocks.sysclk)
        .hclk(clocks.hclk)
        .pclk(clocks.pclk)
        .freeze(&mut p.FLASH);

        let gpioa = p.GPIOA.split(&mut rcc);

        // (Re-)configure PA1 as output
        let mut led = cortex_m::interrupt::free(move |cs| gpioa.pa4.into_push_pull_output(cs));

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, &rcc);
       
        loop {
            led.toggle().ok();
            delay.delay_ms(1_000_u16);
        }
    }

    loop {
        continue;
    }
}


// extern crate rusty_sdk;
// use rusty_sdk::system::mcu_system::*;

// use stm32f0xx_hal::{pac, prelude::*, time::Hertz, timers::*, rcc::Clocks};
// use nb::block;

// pub fn clock_test() {
//     system_init();
//     if let Some(mut p) = pac::Peripherals::take() {
//         let mut rcc_clocks_type_def = RCC_ClocksTypeDef {
//             SYSCLK_Frequency    : 0,
//             HCLK_Frequency      : 0,
//             PCLK_Frequency      : 0,
//             ADCCLK_Frequency    : 0,
//         };
                
//         rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
                
//         let clocks = Clocks {
//             hclk     : rcc_clocks_type_def.HCLK_Frequency.hz(),
//             pclk     : rcc_clocks_type_def.PCLK_Frequency.hz(),
//             sysclk   : rcc_clocks_type_def.SYSCLK_Frequency.hz(),
//         };

//         let mut rcc = p.RCC.configure().sysclk(48.mhz())
//                                         .hclk(48.mhz())
//                                         .pclk(24.mhz())
//                                         .freeze(&mut p.FLASH);

//         let gpioa = p.GPIOA.split(&mut rcc);

//         // (Re-)configure PA4 as output
//         let mut led = cortex_m::interrupt::free(move |cs| gpioa.pa4.into_push_pull_output(cs));

//         let mut timer = Timer::tim1(p.TIM1, Hertz(1), &mut rcc);
        
//         let mut is_led_on = false;

//         loop {
//             if is_led_on {
//                 // Turn off the LED
//                 led.set_low().ok();
//                 is_led_on = false;
//             } else {
//                 // Turn on the LED
//                 led.set_high().ok();
//                 is_led_on = true;
//             }

//             // Wait for the timer to expire
//             block!(timer.wait()).ok();
//         }
//     }

//     loop {
//         continue;
//     }
// }

// #![deny(unsafe_code)]
// #![allow(clippy::empty_loop)]

// // Halt on panic
// use panic_halt as _; // panic handler

// use stm32f0xx_hal::{pac, prelude::*,  rcc::Clocks};

// //________ System library import _________

// extern crate rusty_sdk;
// use rusty_sdk::system::mcu_system::*;

// pub fn clock_test() {

//     system_init(); 

//     if let (Some(dp), Some(_cp)) = (
//         pac::Peripherals::take(),
//         cortex_m::peripheral::Peripherals::take(),
//     ) {
//         let mut flash = dp.FLASH.constrain();
//         // Set up the LED. On the BluePill it's connected to pin PC13.
//         let mut gpioa = dp.GPIOA.split();
//         let mut led = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);

            
//         let mut rcc_clocks_type_def = RCC_ClocksTypeDef
//         {
//             SYSCLK_Frequency    : 0,
//             HCLK_Frequency      : 0,
//             PCLK_Frequency      : 0,
//             ADCCLK_Frequency    : 0,
//         };


//         let clocks = rcc_get_clocks_frequency(&mut rcc_clocks_type_def);
 
//         let clocks = Clocks
//         {
//             hclk: rcc_clocks_type_def.HCLK_Frequency.hz(),
//             pclk: rcc_clocks_type_def.PCLK_Frequency.hz(),
//             sysclk: rcc_clocks_type_def.SYSCLK_Frequency.hz(),
//             adcclk: rcc_clocks_type_def.ADCCLK_Frequency.hz(),
//         };

//         // Set up the system clock. We want to run at 48MHz for this one.
//         let rcc = dp.RCC.constrain();
//         let clocks = rcc
//             .cfgr
//             .sysclk(clocks.sysclk)
//             .hclk(clocks.hclk)
//             .pclk(clocks.pclk)
//             .freeze(&mut flash.acr);

//         // Create a delay abstraction based on general-pupose 32-bit timer TIM2

//         let mut delay = stm32f0xx_hal::timer::FTimerUs::new(dp.TIM2, &clocks).delay();
//         // or
//         // let mut delay = dp.TIM2.delay_us(&clocks);

//         loop {
//             led.set_high();
//             delay.delay_ms(1000_u32);
//             led.set_low();
//             delay.delay_ms(1000_u32);
//         }
//     }

//     loop {}
// }
