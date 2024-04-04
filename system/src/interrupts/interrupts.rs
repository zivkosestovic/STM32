use core::arch::asm;

// ------------------------------------------------------------- PRIVATE MACROS

/* Set bit `_bit` in register `_reg`. */

pub fn interrupt_bit_set(_reg : *mut u32, _bit : u32) 
{
    unsafe 
    {
        *_reg |= 1u32 << _bit;
    }
}


/* Clear bit `_bit` in register `_reg`. */

pub fn interrupt_bit_clear(_reg : *mut u32, _bit : u32) 
{
    unsafe
    {
        *_reg &= !(1u32 << _bit);
    }
}


/* Get IRQ number from IVT. */

pub fn interrupt_get_irq(ivt_val : u32) -> u32 
{
    ivt_val - 16
}





/// IRQ mask.
const INTERRUPT_IRQ_MASK: u32 = 0x1F;

/// Cortex register addresses.
const REGISTER_SCB_SHCRS:       *mut u32 = 0xE000_ED24 as *mut u32;
const REGISTER_STK_CTRL:        *mut u32 = 0xE000_E010 as *mut u32;
const REGISTER_NVIC_ISER_0:     *mut u32 = 0xE000_E100 as *mut u32;
const REGISTER_NVIC_ISER_1:     *mut u32 = 0xE000_E104 as *mut u32;
const REGISTER_NVIC_ISER_2:     *mut u32 = 0xE000_E108 as *mut u32;
const REGISTER_NVIC_ISER_3:     *mut u32 = 0xE000_E10C as *mut u32;
const REGISTER_NVIC_ICER_0:     *mut u32 = 0xE000_E180 as *mut u32;
const REGISTER_NVIC_ICER_1:     *mut u32 = 0xE000_E184 as *mut u32;
const REGISTER_NVIC_ICER_2:     *mut u32 = 0xE000_E188 as *mut u32;
const REGISTER_NVIC_ICER_3:     *mut u32 = 0xE000_E18C as *mut u32;
const REGISTER_NVIC_IPR_0:      *mut u32 = 0xE000_E400 as *mut u32;
const REGISTER_NVIC_SCB_SHPR1:  *mut u32 = 0xE000_ED18 as *mut u32;
const REGISTER_NVIC_SCB_SHPR2:  *mut u32 = 0xE000_ED1C as *mut u32;
const REGISTER_NVIC_SCB_SHPR3:  *mut u32 = 0xE000_ED20 as *mut u32;

/// Vector table numbers.
const IVT_MEM_MANAGE    : u32 = 4;
const IVT_BUS_FAULT     : u32 = 5;
const IVT_USAGE_FAULT   : u32 = 6;
const IVT_SYS_TICK      : u32 = 15;

/// Vector table register bit locations.
const IVT_BIT_TICKINT       : u32 = 1;
const IVT_BIT_MEMFAULTENA   : u32 = 16;
const IVT_BIT_BUSFAULTENA   : u32 = 17;
const IVT_BIT_USGFAULTENA   : u32 = 18;

// ------------------------------------------------ PUBLIC FUNCTION DEFINITIONS

pub fn interrupts_enable() 
{
    interrupts_enable_asm();
}

pub fn interrupts_disable() 
{
    interrupts_disable_asm();
}



pub fn interrupt_enable(interrupt: u32) 
{

    //--------------------------------------------------
    /* 

    TODO Should be handled for M0 core MCUs as they don't have this options
    original C code:
    ---------------------------------
    switch ( interrupt ) {
        #ifdef _INCLUDE_INTERRUPT_CASES_
        case IVT_MEM_MANAGE:
            interrupt_bit_set( REGISTER_SCB_SHCRS, IVT_BIT_MEMFAULTENA );
            break;
        case IVT_BUS_FAULT:
            interrupt_bit_set( REGISTER_SCB_SHCRS, IVT_BIT_BUSFAULTENA );
            break;
        case IVT_USAGE_FAULT:
            interrupt_bit_set( REGISTER_SCB_SHCRS, IVT_BIT_USGFAULTENA );
            break;
        #endif
        case IVT_SYS_TICK:
            interrupt_bit_set( REGISTER_STK_CTRL, IVT_BIT_TICKINT );
            break;

        default: // If none of the above, exit switch.
            break;
    -------------------------------------

    also as variant for conditional compilation we can use this:

    #[cfg(not(feature = "stm32G0"))]
                                                    */
    match interrupt 
    {
        IVT_MEM_MANAGE => interrupt_bit_set(REGISTER_SCB_SHCRS, IVT_BIT_MEMFAULTENA),
        IVT_BUS_FAULT => interrupt_bit_set(REGISTER_SCB_SHCRS, IVT_BIT_BUSFAULTENA),
        IVT_USAGE_FAULT => interrupt_bit_set(REGISTER_SCB_SHCRS, IVT_BIT_USGFAULTENA),
        IVT_SYS_TICK => interrupt_bit_set(REGISTER_STK_CTRL, IVT_BIT_TICKINT),
        _ => (),
    }

    //--------------------------------------------------






    // General exceptions.
    if interrupt >= 112 
    {
        interrupt_bit_set(REGISTER_NVIC_ISER_3, (interrupt_get_irq(interrupt) & INTERRUPT_IRQ_MASK));
    } 
    else if interrupt >= 80 
    {
        interrupt_bit_set(REGISTER_NVIC_ISER_2, (interrupt_get_irq(interrupt) & INTERRUPT_IRQ_MASK));
    } 
    else if interrupt >= 48 
    {
        interrupt_bit_set(REGISTER_NVIC_ISER_1, (interrupt_get_irq(interrupt) & INTERRUPT_IRQ_MASK));
    } 
    else if interrupt >= 16 
    {
        interrupt_bit_set(REGISTER_NVIC_ISER_0, (interrupt_get_irq(interrupt) & INTERRUPT_IRQ_MASK));
    }
}



pub fn interrupt_disable(interrupt: u32) 
{
    //--------------------------------------------------
    /* 
    TODO Should be handled for M0 core MCUs as they don't have this options        
    */
    match interrupt 
    {
        IVT_MEM_MANAGE => interrupt_bit_clear(REGISTER_SCB_SHCRS, IVT_BIT_MEMFAULTENA),
        IVT_BUS_FAULT => interrupt_bit_clear(REGISTER_SCB_SHCRS, IVT_BIT_BUSFAULTENA),
        IVT_USAGE_FAULT => interrupt_bit_clear(REGISTER_SCB_SHCRS, IVT_BIT_USGFAULTENA),
        IVT_SYS_TICK => interrupt_bit_clear(REGISTER_STK_CTRL, IVT_BIT_TICKINT),
        _ => (),
    }
    //--------------------------------------------------




    // General exceptions.
    if interrupt >= 112 
    {
        interrupt_bit_set(REGISTER_NVIC_ICER_3, (interrupt_get_irq(interrupt) & INTERRUPT_IRQ_MASK));
    } 
    else if interrupt >= 80 
    {
        interrupt_bit_set(REGISTER_NVIC_ICER_2, (interrupt_get_irq(interrupt) & INTERRUPT_IRQ_MASK));
    } 
    else if interrupt >= 48 
    {
        interrupt_bit_set(REGISTER_NVIC_ICER_1, (interrupt_get_irq(interrupt) & INTERRUPT_IRQ_MASK));
    } 
    else if interrupt >= 16 
    {
        interrupt_bit_set(REGISTER_NVIC_ICER_0, (interrupt_get_irq(interrupt) & INTERRUPT_IRQ_MASK));
    }
}


// ----------------------------------------------- PRIVATE FUNCTION DEFINITIONS

fn interrupts_enable_asm() 
{
    unsafe { asm!("CPSIE I") };
}

fn interrupts_disable_asm() 
{
    unsafe { asm!("CPSID I") };
}
