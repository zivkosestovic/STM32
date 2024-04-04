use cortex_m::asm;
use stm32h7xx_hal::stm32;
use stm32h7xx_hal::prelude::*;

/* ________ Import System library  _________ */

extern crate rusty_sdk;
use rusty_sdk::system::mcu_system::*;

use rusty_sdk::interrupts::interrupts::*;
use rusty_sdk::interrupts::include::interrupts_mcu::mcu_interrupt_numbers::{INTERRUPTS_TIM2, INTERRUPTS_EXTI15_10}; 

use stm32h7xx_hal::hal::digital::v2::OutputPin;
use stm32h7xx_hal::hal::digital::v2::ToggleableOutputPin;

/* ________ Import stm32l1xx_hal for interrupt function  _________ */

use stm32h7xx_hal::stm32::interrupt;

static mut SWITCH: u8 = 0;

// Timer2 Prescaler: 749; Preload = 39999; Actual Interrupt Time = 1 s
const APB1LENR: u32 = 0xE8 + 0x58024400;

const TIM2_BASE : u32 = 0x40000000;
const TIM2_CR1  : u32 = TIM2_BASE;
const TIM2_PSC  : u32 = TIM2_BASE + 0x28;
const TIM2_ARR  : u32 = TIM2_BASE + 0x2C;
const TIM2_DIER : u32 = TIM2_BASE + 0x0C;
const TIM2_SR   : u32 = TIM2_BASE + 0x10;

fn init_timer2() {
    // Enable Timer2 clock
    // Enable Timer2 clock
    unsafe {
        (*(APB1LENR as *mut u32)) |= 0x00000001;
        asm::nop(); // Delay after peripheral clock enable
    }
    unsafe {
    // Configure Timer2
    (*(TIM2_CR1 as *mut u32)) |= 0x00000000; // Disable Timer2
    (*(TIM2_PSC as *mut u32)) = 0x3e7;
    (*(TIM2_ARR as *mut u32)) = 0xF9FF;

    // Clear update interrupt flag and enable Timer2 interrupt
    (*(TIM2_DIER as *mut u32)) |= 0x00000001;
    (*(TIM2_CR1 as *mut u32)) |= 0x00000001;
    (*(TIM2_SR as *mut u32)) = 0x00000000;

    // Enable Timer2


    /*------------- Test interrupt enable -------------*/

    /* interrupt enable */
    interrupt_enable(INTERRUPTS_TIM2);

    // /* interrupt disable */
    //interrupt_disable(INTERRUPTS_TIM2);

/*------------------------------------------------*/
}
}

#[interrupt]
fn TIM2() {
    
    let dp =  unsafe { stm32::Peripherals::steal() };
    // let cp = cortex_m::Peripherals::take().unwrap();
    
    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOB peripheral
    let gpiob = dp.GPIOA.split(ccdr.peripheral.GPIOA);

    // Configure gpio B pin 0 as a push-pull output.
    let mut led = gpiob.pa4.into_push_pull_output();

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

    init_timer2();

    loop {
        // Your main loop logic here
    }
}

