use cortex_m::asm;
use stm32l1xx_hal::prelude::*;
use stm32l1xx_hal::{
    gpio::{Output, PushPull},
    hal::digital::v2::{OutputPin, ToggleableOutputPin},
    stm32::{Peripherals, interrupt},
};

/* ________ Import System library  _________ */
extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;
use rusty_sdk::interrupts::interrupts::*;
use rusty_sdk::interrupts::include::interrupts_mcu::mcu_interrupt_numbers::{INTERRUPTS_TIM2, INTERRUPTS_EXTI15_10}; 

static mut LED: Option<stm32l1xx_hal::gpio::gpioa::PA4<Output<PushPull>>> = None;
static mut SWITCH: u32 = 0;

// Timer2 Prescaler: 749; Preload = 39999; Actual Interrupt Time = 1 s
const RCC_APB1ENR: u32 = 0x24 + 0x40023800;

const TIM2_BASE : u32 = 0x40000000;
const TIM2_CR1  : u32 = TIM2_BASE;
const TIM2_PSC  : u32 = TIM2_BASE + 0x28;
const TIM2_ARR  : u32 = TIM2_BASE + 0x2C;
const TIM2_DIER : u32 = TIM2_BASE + 0x0C;
const TIM2_SR   : u32 = TIM2_BASE + 0x10;

fn init_timer2() {
    // Enable Timer2 clock
    unsafe {
        (*(RCC_APB1ENR as *mut u32)) |= 0x00000001;
        asm::nop(); // Delay after peripheral clock enable
    }
    
    // Configure Timer2
    unsafe {
        (*(TIM2_CR1 as *mut u32)) |= 0x00000000; // Disable Timer2
        (*(TIM2_PSC as *mut u32)) = 0x1f3;
        (*(TIM2_ARR as *mut u32)) = 0xF9FF;

        // Clear update interrupt flag and enable Timer2 interrupt
        (*(TIM2_DIER as *mut u32)) |= 0x00000001;
        (*(TIM2_CR1 as *mut u32)) |= 0x00000001;
        (*(TIM2_SR as *mut u32)) = 0x00000000;
    }

    /*------------- Test interrupt enable -------------*/
    /* interrupt enable */
    interrupt_enable(INTERRUPTS_TIM2);
    // /* interrupt disable */
    // interrupt_disable(INTERRUPTS_TIM2);
    /*------------------------------------------------*/
}

#[interrupt]
fn TIM2() {
    unsafe {
        if let Some(led) = &mut LED {
            if SWITCH == 0 {
                led.toggle().unwrap();
                SWITCH = 1;
            } else {
                led.toggle().unwrap();
                SWITCH = 0;
            }
        }
    }

    // Clear the update interrupt flag
    unsafe {
        (*(TIM2_SR as *mut u32)) = 0x00000000;
    }
}

pub fn interrupt_test() {
    system_init();
    
    let dp = unsafe { Peripherals::steal() };
    let gpioa = dp.GPIOA.split();
    unsafe {
        LED = Some(gpioa.pa4.into_push_pull_output());
    }
    
    init_timer2();

    loop {
        // Your main loop logic here
    }
}