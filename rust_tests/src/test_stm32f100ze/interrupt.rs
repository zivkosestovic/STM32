use cortex_m::asm;

/* ________ Import LED  _________ */

use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::*;

/* ________ Import System library  _________ */

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

/* ________ Import stm32f4xx_hal for interrupt function  _________ */

use stm32f1xx_hal::pac::interrupt;

/* ________ Import interrupt configuration _________ */

use rusty_sdk::interrupts::interrupts::*;

/* 
    Import TIM2 Timer interrupt number: INTERRUPTS_TIM2
    and import GPIO interrupt number: INTERRUPTS_EXTI15_10
*/

use rusty_sdk::interrupts::include::interrupts_mcu::mcu_interrupt_numbers::{INTERRUPTS_TIM2, INTERRUPTS_EXTI15_10};

static mut SWITCH: u8 = 0;

/* ********* Timer2 registers ********* */

const RCC_APB1ENR : u32 = 0x40000000u32 + 0x00020000u32 + 0x00001000u32 + 0x1C;

const TIM2_BASE : u32 = 0x40000000;

const TIM2_CR1 : u32 = TIM2_BASE;
const TIM2_PSC : u32 = TIM2_BASE + 0x28;
const TIM2_ARR : u32 = TIM2_BASE + 0x2C;
const TIM2_DIER : u32 = TIM2_BASE + 0x0C;
const TIM2_SR   : u32 = TIM2_BASE + 0x10;

fn init_timer2() {
    unsafe 
    {
        *(RCC_APB1ENR as *mut u32) |= 0x00000001;   // RCC_APB1ENR.TIM2EN = 1
        *(TIM2_CR1 as *mut u32) |= 0x00000000;      // TIM2_CR1.CEN = 0;
        *(TIM2_PSC as *mut u32) = 0x176;           // TIM2_PSC = 9999;
        *(TIM2_ARR as *mut u32) = 0xF9FF;           // TIM2_ARR = 63999;
    }

/*------------- Test interrupt enable -------------*/

    /* interrupt enable */
    interrupt_enable(INTERRUPTS_TIM2);

    // /* interrupt disable */
    //interrupt_disable(INTERRUPTS_TIM2);

/*------------------------------------------------*/


    unsafe
    {
        *(TIM2_DIER as *mut u32) |= 0x00000001;      // TIM2_DIER.UIE = 1;
        *(TIM2_CR1 as *mut u32) |= 0x00000001;       // TIM2_CR1.CEN = 1;
    }

}


#[interrupt]
fn TIM2() {
    
        let mut p = unsafe { pac::Peripherals::steal() };

        let mut gpioa = p.GPIOA.split();

        // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
        // in order to configure the port. For pins 0-7, crl should be passed instead.
        let mut led = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
        // Configure the syst timer to trigger an update every second
    // On for 1s, off for 1s.
    
    unsafe {
        if SWITCH == 0 {
            led.set_high();
            SWITCH = 1;
        } else  {
            led.set_low();
            SWITCH = 0;
        }
    }

    // Clear the update interrupt flag
    unsafe {
        (*(TIM2_SR as *mut u32)) = 0x00000000;
    }
}

pub fn interrupt_test() {
    system_init();
    // Set up the Timer2 interrupt
    init_timer2();

    loop {
        // Your main loop logic here
    }
}