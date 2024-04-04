use core::arch::asm;

use crate::common::common_header::*;
use crate::core_header::*;
use crate::mcu_headers::mcu::*;

const ADDRESS_SCB_AIRCR     : u32 = 0xE000ED0C;
const SCB_AIRCR_SYSRESETREQ : u32 = 2;

pub struct RCC_ClocksTypeDef {
    pub SYSCLK_Frequency    : u32,
    pub HCLK_Frequency      : u32,
    pub PCLK_Frequency     : u32,
    pub TPCLK_Frequency     : u32,
}

const AHBPrescTable: [u32; 8] = [1, 2, 4, 8, 32, 64, 128, 256];
const APBPrescTable: [u8; 4] = [2, 4, 8, 16];

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
    /* Enable Syscfg clock */
    reg_value_set_bit(RCC_APBENR1 as *mut u32, RCC_APBENR1_PWREN_Pos);
    /* Enable PWR clock */
    reg_value_set_bit(RCC_APBENR2 as *mut u32, RCC_APBENR2_SYSCFGEN_Pos);
    /* Reset PLLSRC[1:0] */
    reg_value_clear_mask(RCC_PLLCFGR as *mut u32, !RCC_PLLCFGR_PLLSRC_Msk);
    /* Set HSION bit */
    reg_value_set_bit(RCC_CR as *mut u32, RCC_CR_HSION_Pos);
    /* Set latency to RANGE 2 (VOS1 = 1) */
    reg_value_set_bit(PWR_CR1 as *mut u32, 0x00000400);
    /* Reset everything but SWS(System clock switch status) */
    reg_value_clear_mask(RCC_CFGR as *mut u32, 0x00000038);
    /* Reset HSEON, CSSON and PLLON bits */
    reg_value_clear_mask(RCC_CR as *mut u32, 0xFEF2FFFF);
}
//*****************************************************************************
// Sets correct flash latency!!!
// ----------------------               | ------------------------
//      RANGE 1         |               | |     RANGE 2          |
// -------||------------|               | |-------||-------------|
//    WS  ||    HCLK    |               | |   WS ||    HCLK      |
// -------||------------|               | |-------||-------------|
//    0   ||   <= 24MHz |               | |   0   ||   <= 8MHz   |
// -------||----------------------------| |-------||----------------------------
//    1   ||   24MHz < HCLK <= 48MHz   || |   1   ||   8MHz < HCLK <= 16MHz    |
// -------||------------|-------------- | |-------||-------------|--------------
//    2   ||   > 48MHz  |               | |   2   ||      /      |
// -------||------------|               | |-------||-------------|
//
// Flash latency is dependent on HCLK only!!!
// Range is selected by writing 1 or 0 to VOS bit of PWR1_CR1 register.
// This bit is set via edit project.
//*****************************************************************************
pub fn set_core_latency(core_voltage_value: u32, hclk_freq: u32) {
    /* latency depends on HCLK frequency */
    if core_voltage_value & PWR_CR1_VOS_1 != 0 { /* range 2 */
        if hclk_freq <= 8000 {
            while reg_value_get(FLASH_ACR as *mut u32) & !FLASH_ACR_LATENCY_Msk != !FLASH_ACR_LATENCY_Msk {}
        } else {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_1);
            while reg_value_get(FLASH_ACR as *mut u32) & FLASH_ACR_LATENCY_1 != FLASH_ACR_LATENCY_1 {}
        }
        reg_value_set(PWR_CR1 as *mut u32, PWR_CR1_VOS_1);
    } else if core_voltage_value & PWR_CR1_VOS_0 != 0 { /* range 1 */
        /* first set the VOS value */
        reg_value_set(PWR_CR1 as *mut u32, PWR_CR1_VOS_0);
        /* wait for voltage scaling flag to be cleared */
        while reg_value_get_bit(PWR_SR2 as *mut u32, PWR_SR2_VOSF_Pos) != 0 {}
        /* then set the flash latency value */
        if hclk_freq <= 24000 {
            while reg_value_get(FLASH_ACR as *mut u32) & !FLASH_ACR_LATENCY_Msk != !FLASH_ACR_LATENCY_Msk {}
        } else if 24000 < hclk_freq && hclk_freq <= 48000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_1);
            while reg_value_get(FLASH_ACR as *mut u32) & FLASH_ACR_LATENCY_1 != FLASH_ACR_LATENCY_1 {}
        } else {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_2);
            while reg_value_get(FLASH_ACR as *mut u32) & FLASH_ACR_LATENCY_2 != FLASH_ACR_LATENCY_2 {}
        }
    }
}
pub fn rcc_get_clocks_frequency(rcc_clocks: &mut RCC_ClocksTypeDef) {
    let mut tmp: u32;
    let presc: u8;
    let presc1: u8;
    let presc2: u8;

    rcc_clocks.HCLK_Frequency = FOSC_KHZ_VALUE * 1000;

    /*------ Compute HCLK, PCLK, and TPCLK clocks frequencies ------*/

    /* Get HCLK prescaler */
    tmp = reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_HPRE_Msk;
    tmp >>= RCC_CFGR_HPRE_Pos;

    if tmp & 0x08 != 0 {
        presc = 1;
    } else {
        tmp &= 0x07;
        presc = AHBPrescTable[tmp as usize] as u8;
    }

    /* SYSCLK clock frequency */
    rcc_clocks.SYSCLK_Frequency = rcc_clocks.HCLK_Frequency * presc as u32;

    /* Get PCLK prescaler */
    tmp = reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_PPRE_Msk;
    tmp >>= RCC_CFGR_PPRE_Pos;

    if tmp == 0 {
        // PCLK clock prescaler is 1
        presc1 = 1;
        // TPCLK prescaler is 1
        presc2 = 1;
    } else {
        tmp &= 0x03;
        // PCLK clock prescaler
        presc1 = APBPrescTable[tmp as usize];
        // TPCLK prescaler is 2
        presc2 = 2;
    }

    /* PCLK clock frequency */
    rcc_clocks.PCLK_Frequency = rcc_clocks.HCLK_Frequency / presc1 as u32;

    /* Get TPCLK prescaler */
    /* TPCLK clock is calculated in the following way:
      If APB Prescaler = 1 then TPCLK prescaler is x1
      else if APB Presc > 1 then TPCLK presc is x2 */
    rcc_clocks.TPCLK_Frequency = rcc_clocks.PCLK_Frequency * presc2 as u32;
}
//*****************************************************************************
//
// Checks if activated clocks are stable.
//
//*****************************************************************************
pub fn is_clock_stable() {
    if reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_HSION_Pos) != 0 {                  /* if HSI enabled */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_HSIRDY_Pos) == 0 {
            /* Wait for HSIRDYF = 1 (HSI is ready) */
        }
    }
    if reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_HSEON_Pos) != 0 {                  /* if HSE enabled */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_HSERDY_Pos) == 0 {
            /* Wait for HSERDY = 1 (HSE is ready) */
        }
    }
    if reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_PLLON_Pos) != 0 {                  /* if PLL enabled */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_PLLRDY_Pos) == 0 {
            /* Wait for PLL1RDY = 1 (PLL is ready) */
        }    
    }
    if reg_value_get_bit(RCC_CSR as *mut u32, RCC_CSR_LSION_Pos) != 0 {                /* if LSI enabled */
        while (reg_value_get_bit(RCC_CSR as *mut u32, RCC_CSR_LSIRDY_Pos) == 0) {
            /* Wait for LSIRDY = 1 (LSI is ready) */
        }
    }
    if reg_value_get_bit(RCC_BDCR as *mut u32, RCC_BDCR_LSEON_Pos) != 0 {              /* if LSE enabled */
        while reg_value_get_bit(RCC_BDCR as *mut u32, RCC_BDCR_LSERDY_Pos) == 0 {
            /* Wait for LSERDY = 1 (LSE is ready) */
        }

    }
}
pub fn system_init() {
    /* set correct core latency */
    set_core_latency(VALUE_PWR_CR1, FOSC_KHZ_VALUE);

    /* turn prefetch on */
    reg_value_set_bit(FLASH_ACR as *mut u32, FLASH_ACR_PRFTEN_Pos);

    /* set default clock settings */
    system_clock_set_default();

    /* now set real values */
    reg_value_clear_set(RCC_PLLCFGR as *mut u32, VALUE_RCC_PLLSYSCFGR);
    reg_value_clear_set(RCC_CR as *mut u32, VALUE_RCC_CR);
    reg_value_clear_set(RCC_CFGR as *mut u32, VALUE_RCC_CFGR);
    reg_value_clear_set(RCC_BDCR as *mut u32, VALUE_RCC_BDCR);
    reg_value_clear_set(RCC_CSR as *mut u32, VALUE_RCC_CSR);
    reg_value_clear_set(RCC_CCIPR as *mut u32, VALUE_RCC_CCIPR);

    /* Wait for clock to stabilize */
    is_clock_stable();

    /* Wait till SYSCLK is stabilized (depending on selected clock) */    
    while (reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_SWS_Msk) != ((VALUE_RCC_CFGR << 3) & RCC_CFGR_SWS_Msk) {
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
            "subs {0}, #1",
            "nop",
            "cmp {0}, #0",
            "ble 2f",
            "B 2b",
            "2:",
            inout(reg) cycle_num,
        );
    }
}

#[inline(never)]
pub fn Delay_us(time_us: u32) 
{
    /*
     * Delay for STM32G070RB - default NECTO setup
     */
    Delay_Cyc(time_us * get_clock_value(FOSC_KHZ_VALUE) / 6 );
}


#[inline(never)]
pub fn Delay_ms(time_ms: u32) 
{

    /*
     * Delay for STM32G070RB - default NECTO setup
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