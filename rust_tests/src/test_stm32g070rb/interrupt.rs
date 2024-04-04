use cortex_m::asm;
use stm32g0xx_hal::prelude::*;
use stm32g0xx_hal::{
    gpio::{Output, PushPull},
    hal::digital::v2::{OutputPin, ToggleableOutputPin},
    stm32::{Peripherals, interrupt},
};

/* ________ Import System library  _________ */
extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;
use rusty_sdk::interrupts::interrupts::*;
use rusty_sdk::interrupts::include::interrupts_mcu::mcu_interrupt_numbers::{INTERRUPTS_TIM3, INTERRUPTS_EXTI0_1}; 

static mut SWITCH: u32 = 0;

// Timer3 Prescaler: 249; Preload = 63999; Actual Interrupt Time = 1 s
const RCC_APBENR1: u32 = 0x3C + 0x40021000;

const TIM3_BASE : u32 = 0x40000400;
const TIM3_CR1  : u32 = TIM3_BASE;
const TIM3_PSC  : u32 = TIM3_BASE + 0x28;
const TIM3_ARR  : u32 = TIM3_BASE + 0x2C;
const TIM3_DIER : u32 = TIM3_BASE + 0x0C;
const TIM3_SR   : u32 = TIM3_BASE + 0x10;

fn init_timer3() {
    // Enable Timer3 clock
    unsafe {
        (*(RCC_APBENR1 as *mut u32)) |= 0x00000001;
        asm::nop(); // Delay after peripheral clock enable
    }
    
    // Configure Timer2
    unsafe {
        (*(TIM3_CR1 as *mut u32)) |= 0x00000000; // Disable Timer3
        (*(TIM3_PSC as *mut u32)) = 0xf9;
        (*(TIM3_ARR as *mut u32)) = 0xF9FF;

        // Clear update interrupt flag and enable Timer3 interrupt
        (*(TIM3_DIER as *mut u32)) |= 0x00000001;
        (*(TIM3_CR1 as *mut u32)) |= 0x00000001;
        (*(TIM3_SR as *mut u32)) = 0x00000000;
    }

    /*------------- Test interrupt enable -------------*/
    /* interrupt enable */
    interrupt_enable(INTERRUPTS_TIM3);
    // /* interrupt disable */
    // interrupt_disable(INTERRUPTS_TIM3);
    /*------------------------------------------------*/
}

#[interrupt]
fn TIM3() {
    
    let dp =  unsafe { Peripherals::steal() };

    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output();

    // On for 1s, off for 1s.
    
    unsafe {
        if SWITCH == 0 {
            led.set_high().ok();
            SWITCH = 1;
        } else  {
            led.set_low().ok();
            SWITCH = 0;
        }
    }
    // Clear the update interrupt flag
    unsafe {
        (*(TIM3_SR as *mut u32)) = 0x00000000;
    }
}


pub fn interrupt_test() {
    system_init();

    init_timer3();

    loop {
        // Your main loop logic here
    }
}
