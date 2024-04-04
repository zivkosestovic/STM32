use core::arch::asm;

use crate::common::common_header::*;
use crate::core_header::*;
use crate::mcu_headers::mcu::*;

const ADDRESS_SCB_AIRCR     : u32 = 0xE000ED0C;

const SCB_AIRCR_SYSRESETREQ : u32 = 2;

pub struct RCC_ClocksTypeDef
{
    pub SYSCLK_Frequency    : u32,
    pub HCLK_Frequency      : u32,
    pub PCLK_Frequency      : u32,
    pub ADCCLK_Frequency    : u32,
}

const APBAHBPrescTable : [u8; 16] = [0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 6, 7, 8, 9];
const ADCPrescTable    : [u8;  2] = [2, 4];

//*****************************************************************************
//
// Resets the device.
//
// This function will perform a software reset of the entire device. The
// processor and all peripherals are reset and all device registers will
// return to their default values (with the exception of the reset cause
// register, which will maintain its current value but have the software reset
// bit set as well).
//
// \return This function does not return anything.
//
//*****************************************************************************
pub fn system_reset()
{
    // Perform a software reset request. This will cause the device to reset,
    // no further code is executed.

    reg_value_clear_set(ADDRESS_SCB_AIRCR as *mut u32, 0x05FA0000 | (1 << SCB_AIRCR_SYSRESETREQ));

    // The device should have reset, so this should never be reached. Just in
    // case, loop forever.

    loop{};
}

/*
   @brief Resets the RCC clock configuration to the default reset state.
   @note   The default reset state of the clock configuration is given below:
              - HSI ON and used as system clock source
              - HSE, PLL and PLLI2S OFF
              - AHB, APB1 and APB2 prescaler set to 1.
              - CSS, MCO1 and MCO2 OFF
              - All interrupts disabled (not used)
   @note   This function doesn't modify the configuration of the
              - Peripheral clocks
              - LSI, LSE and RTC clocks
   @param None
   @retval None
*/

pub fn system_clock_set_default() {
    /* Set HSION bit */
    reg_value_set_bit(RCC_CR as *mut u32, RCC_CR_HSION_Pos);

    /* Reset SW[1:0], HPRE[3:0], PPRE[2:0], ADCPRE and MCOSEL[2:0] bits */
    reg_value_clear_mask(RCC_CFGR as *mut u32, 0xF8FFB80C);

    /* Reset HSEON, CSSON and PLLON bits */
    reg_value_clear_mask(RCC_CR as *mut u32, 0xFEF6FFFF);
    
    /* Reset HSEBYP bit */
    reg_value_clear_bit(RCC_CR as *mut u32, RCC_CR_HSEBYP_Pos);

    /* Reset PLLSRC, PLLXTPRE and PLLMUL[3:0] bits */
    reg_value_clear_mask(RCC_CFGR as *mut u32, 0xFFC0FFFF);
    
    /* Reset PREDIV1[3:0] bits */
    reg_value_clear_mask(RCC_CFGR2 as *mut u32, !RCC_CFGR2_PREDIV_Msk);

    /* Reset USARTSW[1:0], I2CSW, CECSW and ADCSW bits */
    reg_value_clear_mask(RCC_CFGR3 as *mut u32, 0xFFFFFEAC);

    /* Reset HSEBYP bit */
    reg_value_clear_bit(RCC_CR2 as *mut u32, RCC_CR2_HSI14ON_Pos);
}

pub fn rcc_get_clocks_frequency(rcc_clocks: &mut RCC_ClocksTypeDef) {
    let mut tmp   : u32;
    let mut presc : u8;

    rcc_clocks.HCLK_Frequency = FOSC_KHZ_VALUE * 1000;

    /*------ Compute HCLK, PCLK, and ADCCLK clocks frequencies ------*/

    /* Get HCLK prescaler */
    tmp = reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_HPRE_Msk;
    tmp >>= RCC_CFGR_HPRE_Pos;
    presc = APBAHBPrescTable[tmp as usize];

    /* SYSCLK clock frequency */
    rcc_clocks.SYSCLK_Frequency = rcc_clocks.HCLK_Frequency << presc;

    /* Get PCLK prescaler */
    tmp = reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_PPRE_Msk;
    tmp >>= RCC_CFGR_PPRE_Pos;
    presc = APBAHBPrescTable[tmp as usize];

    /* PCLK clock frequency */
    rcc_clocks.PCLK_Frequency = rcc_clocks.HCLK_Frequency >> presc;

    // Get ADC clock frequency
    if reg_value_get(RCC_CR as *mut u32) & RCC_CR2_HSI14ON_Pos != 0 {
        // ADC Clock 14MHz
        rcc_clocks.ADCCLK_Frequency = 14_000_000;
    } else if rcc_clocks.PCLK_Frequency > 24_000_000 {
        rcc_clocks.ADCCLK_Frequency = rcc_clocks.PCLK_Frequency >> 2;
    } else {
        rcc_clocks.ADCCLK_Frequency = rcc_clocks.PCLK_Frequency >> 1;
    }    
}

pub fn system_init() {
    let mut tmp : u32;
    let sys_clk : u32;
    let presc   : u32;

    tmp = VALUE_RCC_CFGR & 0x000000F0;
    tmp >>= 4;
    presc = APBAHBPrescTable[tmp as usize] as u32;

    sys_clk = FOSC_KHZ_VALUE << presc;

    if sys_clk > 24000 {        
        reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY);
    }
    else {
        reg_value_clear_mask(FLASH_ACR as *mut u32, 0xFFFFFFF8);
    }

    system_clock_set_default();

    reg_value_clear_set(RCC_CFGR as *mut u32,  VALUE_RCC_CFGR);            /* set clock configuration register */
    reg_value_clear_set(RCC_CFGR2 as *mut u32, VALUE_RCC_CFGR2);           /* set clock configuration register 2 */
    reg_value_clear_set(RCC_CR as *mut u32,    VALUE_RCC_CR & 0x000FFFFF); /* do not start PLLs yet */
    reg_value_clear_set(RCC_CSR as *mut u32,   VALUE_RCC_CSR);
    reg_value_clear_set(RCC_CR2 as *mut u32,   VALUE_RCC_CR2);

    if VALUE_RCC_CR & (1 << RCC_CR_HSION_Pos) != 0 { // if HSI enabled
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_HSIRDY_Pos) == 0 {
            // Wait for RCC_CR_HSIRDY_Pos = 1 (HSI is ready)
        }
    } 

    if VALUE_RCC_CR & (1 << RCC_CR_HSEON_Pos) != 0 { /* if HSE enabled */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_HSERDY_Pos) == 0 {
            /* Wait for HSERDY = 1 (HSE is ready) */
        }
    }

    if VALUE_RCC_CR & (1 << RCC_CR_PLLON_Pos) != 0 { /* if PLL enabled */
        reg_value_set_bit(RCC_CR as *mut u32, RCC_CR_PLLON_Pos); /* PLL On */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_PLLRDY_Pos) == 0 {
            /* Wait for PLL1RDY = 1 (PLL is ready) */
        }
    }

    /* Wait till SYSCLK is stabilized (depending on selected clock) */    
    while (reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_SWS_Msk) != ((VALUE_RCC_CFGR << 2) & RCC_CFGR_SWS_Msk) {
    }
}

// ==================== DELAYS ======================

fn get_clock_value(_clock : u32) -> u32
{
    _clock / 1000
}

#[inline(never)]
#[no_mangle]
#[link_section = ".ramfunc"]

pub fn Delay_Cyc(mut cycle_num : u32)
{
    unsafe
    {
        asm!(
             
            "2:",
            "    subs {0}, {0}, #1",
            "    nop",
            "    cmp {0}, #0",
            "    ble 2f",
            "    B 2b",
            "2:",
            inout(reg) cycle_num,
        );
        // Use the value to avoid the "unused_assignments" warning
        let _ = cycle_num;
    }
}

#[inline(never)]
pub fn Delay_us(time_us: u32) 
{
    /*
     * Delay for STM32F042C6 - default NECTO setup
     */
    Delay_Cyc(time_us * get_clock_value(FOSC_KHZ_VALUE) / 7 );
}


#[inline(never)]
pub fn Delay_ms(time_ms: u32) 
{

    /*
     * Delay for STM32F042C6 - default NECTO setup
     */

    Delay_us(time_ms * 1000);
}

#[inline(never)]
pub fn Delay_Advanced_ms(time_ms: u32, current_fosc_kHz: u32) 
{

}

// Functions for specific delays in microseconds
pub fn delay_1us() {
    Delay_us(1);
}

pub fn delay_5us() {
    Delay_us(5);
}

pub fn delay_6us() {
    Delay_us(6);
}

pub fn delay_9us() {
    Delay_us(9);
}

pub fn delay_10us() {
    Delay_us(10);
}

pub fn delay_22us() {
    Delay_us(22);
}

pub fn delay_50us() {
    Delay_us(50);
}

pub fn delay_55us() {
    Delay_us(55);
}

pub fn delay_60us() {
    Delay_us(60);
}

pub fn delay_64us() {
    Delay_us(64);
}

pub fn delay_70us() {
    Delay_us(70);
}

pub fn delay_80us() {
    Delay_us(78);
}

pub fn delay_410us() {
    Delay_us(410);
}

pub fn delay_480us() {
    Delay_us(480);
}

pub fn delay_500us() {
    Delay_us(498);
}

pub fn delay_5500us() {
    Delay_us(5500);
}

// Functions for delays in milliseconds
pub fn delay_1ms() {
    Delay_ms(1);
}

pub fn delay_5ms() {
    Delay_ms(5);
}

pub fn delay_8ms() {
    Delay_ms(8);
}

pub fn delay_10ms() {
    Delay_ms(10);
}

pub fn delay_100ms() {
    Delay_ms(100);
}

pub fn delay_1sec() {
    Delay_ms(1000);
}