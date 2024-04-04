const PERIPH_BASE : u32 = 0x40000000u32;
const AHB1PERIPH_BASE : u32 = PERIPH_BASE + 0x00020000u32;
const RCC_BASE : u32 = AHB1PERIPH_BASE + 0x3800u32;

const GPIOA_BASE : u32 = AHB1PERIPH_BASE; 

const GPIOA_MODER   : u32 = GPIOA_BASE;                               
const GPIOA_OTYPER  : u32 = GPIOA_BASE + 0x04;                        
const GPIOA_OSPEEDR : u32 = GPIOA_BASE + 0x08;                        
const GPIOA_PUPDR   : u32 = GPIOA_BASE + 0x0C;                        
const GPIOA_IDR     : u32 = GPIOA_BASE + 0x10;                        
const GPIOA_ODR     : u32 = GPIOA_BASE + 0x14;                        
const GPIOA_BSRR    : u32 = GPIOA_BASE + 0x18;                        
const GPIOA_LCKR    : u32 = GPIOA_BASE + 0x1C;                        
const GPIOA_AFRL    : u32 = GPIOA_BASE + 0x20;                        
const GPIOA_AFRH    : u32 = GPIOA_BASE + 0x24;                        


const RCC_AHB1ENR : u32 = RCC_BASE + 0x30;


pub fn digital_out_init_PA6()
{
    unsafe
    {
        *(RCC_AHB1ENR     as *mut u32)   = 0x100001;

        *(GPIOA_MODER       as *mut u32) = 0xA8001000;    
        *(GPIOA_OTYPER      as *mut u32) = 0x0;
        *(GPIOA_OSPEEDR     as *mut u32) = 0xC003000;
        *(GPIOA_PUPDR       as *mut u32) = 0x64000000;
        *(GPIOA_IDR         as *mut u32) = 0x0;
        *(GPIOA_ODR         as *mut u32) = 0x0;
        *(GPIOA_BSRR        as *mut u32) = 0x0;
        *(GPIOA_LCKR        as *mut u32) = 0x0;
        *(GPIOA_AFRL        as *mut u32) = 0x0;
        *(GPIOA_AFRH        as *mut u32) = 0x0;
    
    }
}


pub fn digital_out_high()
{
    unsafe
    {
        *(GPIOA_IDR         as *mut u32) = 0x80C4;
        *(GPIOA_ODR         as *mut u32) = 0x40;
    }

}


pub fn digital_out_low()
{
    unsafe
    {
        *(GPIOA_IDR         as *mut u32) = 0x8084;
        *(GPIOA_ODR         as *mut u32) = 0x0;
        
    }
}


pub fn gpio_test()
{
    digital_out_init_PA6();
    digital_out_high();
}
