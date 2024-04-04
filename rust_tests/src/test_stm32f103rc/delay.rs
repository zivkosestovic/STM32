/* ________ GPIOA PA5 set registers _________ */

const PERIPH_BASE : u32 = 0x40000000u32;
const APB2PERIPH_BASE : u32 = PERIPH_BASE + 0x00010000u32;
const GPIOA_BASE : u32 = APB2PERIPH_BASE + 0x00000800u32;

const GPIOA_CRL : u32 = GPIOA_BASE;

const GPIOA_ODR : u32 = GPIOA_BASE + 0x0C;


const AHBPERIPH_BASE : u32 = PERIPH_BASE + 0x00020000u32;
const RCC_BASE : u32 = AHBPERIPH_BASE + 0x00001000u32; 

const RCC_APB2ENR : u32 = RCC_BASE + 0x18u32;

const RCC_APB2ENR_IOPAEN_Pos : u32 = 2u32;
const RCC_APB2ENR_IOPAEN_Msk : u32 = 0x1u32 << RCC_APB2ENR_IOPAEN_Pos;
const RCC_APB2ENR_IOPAEN : u32 = RCC_APB2ENR_IOPAEN_Msk;


const GPIO_CRL_MODE5_Pos : u32 = 20;
const GPIO_CRL_MODE5_Msk : u32 = 0x3u32 << GPIO_CRL_MODE5_Pos; 
const GPIO_CRL_MODE5 : u32 = GPIO_CRL_MODE5_Msk;


const GPIO_CRL_CNF5_Pos : u32 = 22u32;                             
const GPIO_CRL_CNF5_Msk : u32 = (0x3u32 << GPIO_CRL_CNF5_Pos);       
const GPIO_CRL_CNF5 : u32 = GPIO_CRL_CNF5_Msk;                 



const GPIO_CRL_MODE5_0 : u32 = 0x1u32 << GPIO_CRL_MODE5_Pos;


const GPIO_ODR_ODR5_Pos : u32 = 5u32;                              
const GPIO_ODR_ODR5_Msk : u32 = 0x1u32 << GPIO_ODR_ODR5_Pos;
const GPIO_ODR_ODR5 : u32 = GPIO_ODR_ODR5_Msk;                


pub fn GPIOA_init()
{
    unsafe
    {
        *(RCC_APB2ENR as *mut u32) |= RCC_APB2ENR_IOPAEN;

        *(GPIOA_CRL as *mut u32) &= !(GPIO_CRL_MODE5 | GPIO_CRL_CNF5); 

        *(GPIOA_CRL as *mut u32) |= GPIO_CRL_MODE5_0; 
    }
}



pub fn GPIOA_Toggle()
{
    unsafe
    {
        *(GPIOA_ODR as *mut u32) ^= GPIO_ODR_ODR5;
    }
}



/* ________ Import System library _________ */

extern crate rusty_sdk;

use rusty_sdk::system::mcu_system::*;


/* ________ Import GPIOA test _________ */
// use super::gpioa_test::*;

pub fn delay_test()
{
    /* --------- Init GPIOA --------- */

    GPIOA_init();

    /* --------- Set Clock --------- */

    system_init(); // Initialization of Clock


    /* --------- Light Show --------- */

    loop 
    {
        unsafe
        {
            *(GPIOA_ODR as *mut u32) ^= GPIO_ODR_ODR5;
        }

        Delay_ms(1000);
    }
}
