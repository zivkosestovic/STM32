use core::arch::asm;

use crate::common::common_header::*;
use crate::core_header::*;
use crate::mcu_headers::mcu::*;

const ADDRESS_SCB_AIRCR     : u32 = 0xE000ED0C;
const SCB_AIRCR_SYSRESETREQ : u8  = 2;

// Voltage range
// 2.7 to 3.6 V
const VR_2700_3600: u32 = 3;
// Voltage range
// 2.4 to 2.7 V
const VR_2400_2700: u32 = 2;
// Voltage range
// 2.1 to 2.4 V
const VR_2100_2400: u32 = 1;
// Voltage range
// 1.8 to 2.1 V
const VR_1800_2100: u32 = 0;

pub struct RCC_ClocksTypeDef {
    pub hclk_frequency   : u32,
    pub sysclk_frequency : u32,
    pub pclk1_frequency  : u32,
    pub pclk2_frequency  : u32,
}

const APBAHBPRESC_TABLE : [u8; 16] = [0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 6, 7, 8, 9];

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
    reg_value_set_bit(RCC_CR as *mut u32, RCC_CR_HSION_POS);

    /* Reset CFGR register */
    reg_value_clear(RCC_CFGR as *mut u32);

    /* Reset HSEON, CSSON and PLLON bits */
    reg_value_clear_mask(RCC_CR as *mut u32, 0xFEF6FFFF);

    /* Reset PLLSRC, PLLXTPRE, PLLM and USBPRE bits */
    reg_value_clear_set(RCC_PLLCFGR as *mut u32, 0x24003010);

    /* Reset HSEBYP bit */
    reg_value_clear_bit(RCC_CR as *mut u32, RCC_CR_HSEBYP_POS);

    // Disable all interrupts and clear pending bits
    reg_value_clear(RCC_CIR as *mut u32);
}

pub fn rcc_get_clocks_frequency(rcc_clocks: &mut RCC_ClocksTypeDef) {
    let mut tmp   : u32;
    let mut presc : u8;

    rcc_clocks.hclk_frequency = FOSC_KHZ_VALUE * 1000;

    /*------ Compute HCLK, PCLK1, and PCLK2 clocks frequencies ------*/
    
    /* Get HCLK prescaler */
    tmp = reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_HPRE_MSK;
    tmp >>= RCC_CFGR_HPRE_POS;
    presc = APBAHBPRESC_TABLE[tmp as usize];

    /* HCLK clock frequency */
    rcc_clocks.sysclk_frequency = rcc_clocks.hclk_frequency << presc;

    /* Get PCLK1 prescaler */
    tmp = reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_PPRE1_MSK;
    tmp >>= RCC_CFGR_PPRE1_POS;
    presc = APBAHBPRESC_TABLE[tmp as usize];

    // PCLK1 clock frequency
    rcc_clocks.pclk1_frequency = rcc_clocks.hclk_frequency >> presc;

    // Get PCLK2 prescaler
    tmp = reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_PPRE2_MSK;
    tmp >>= RCC_CFGR_PPRE2_POS;
    presc = APBAHBPRESC_TABLE[tmp as usize];

    // PCLK2 clock frequency
    rcc_clocks.pclk2_frequency = rcc_clocks.hclk_frequency >> presc;
}

/**
   @brief Sets the overdrive mode.
   @note Used to get max mcu frequency.
   @param None
   @retval None
  */

pub fn enable_overdrive_mode() {
    // Enable power clock
    reg_value_set_bit(RCC_APB1ENR as *mut u32, RCC_APB1ENR_PWREN_POS);

    // Set overdrive bit
    reg_value_set_bit(PWR_CR1 as *mut u32, PWR_CR1_ODEN_POS); // ODEN

    // Wait for overdrive ready flag to be set
    while reg_value_get_bit(PWR_CSR1 as *mut u32, PWR_CSR1_ODRDY_POS) == 0 // ODRDY
    {   }

    // Enable overdrive switching
    reg_value_set_bit(PWR_CR1 as *mut u32, PWR_CR1_ODSWEN_POS); //ODSWEN
    
    // Wait for overdrive switch ready flag to be set
    while reg_value_get_bit(PWR_CSR1 as *mut u32, PWR_CSR1_ODSWRDY_POS) == 0 //ODSWRDY
    {   }
}

const NVIC_PRIORITYGROUP_0   : u32 = 0x00000007;
const NVIC_PRIORITYGROUP_1   : u32 = 0x00000006;
const NVIC_PRIORITYGROUP_2   : u32 = 0x00000005;
const NVIC_PRIORITYGROUP_3   : u32 = 0x00000004;
const NVIC_PRIORITYGROUP_4   : u32 = 0x00000003;

const SCB_AIR_VECTORKEY_POS  : u32 = 16;
const SCB_AIR_VECTORKEY_MASK : u32 = 0xFFFF << SCB_AIR_VECTORKEY_POS;
const SCB_AIR_PRIGROUP_POS   : u32 = 8;
const SCB_AIR_PRIGROUP_MASK  : u32 = 0x7 << SCB_AIR_PRIGROUP_POS;

/* Set Priority Grouping
 *
 *   The function sets the priority grouping field using the required unlock sequence.
 *   The parameter PriorityGroup is assigned to the field SCB->AIRCR [10:8] PRIGROUP field.
 *   Only values from 0..7 are used.
 *   In case of a conflict between priority grouping and available
 *   priority bits (__NVIC_PRIO_BITS), the smallest possible priority group is set.
 *
 *   Param: - priorityGroup Priority grouping field.
 *   Return: None
 */

pub fn nvic_set_priority_grouping(priority_group: u32) {
    let priority_group_temp = priority_group & 0x07;
    let mut reg_val: u32;

    unsafe {
        reg_val = *(ADDRESS_SCB_AIRCR as *const u32);
        reg_val &= !(SCB_AIR_VECTORKEY_MASK | SCB_AIR_PRIGROUP_MASK);
        reg_val |= (0x5FA << SCB_AIR_VECTORKEY_POS) | (priority_group_temp << 8);
        *(ADDRESS_SCB_AIRCR as *mut u32) = reg_val;
    }
}

// /* Get Priority Grouping
//  *
//  *   The function reads the priority grouping field from the NVIC Interrupt Controller.
//  *
//  *   Return:
//  *   Priority grouping field (SCB_AIRCR [10:8] PRIGROUP field).
//  */

pub fn nvic_get_priority_grouping() -> u32 {
    let reg_val: u32;

    unsafe {
        reg_val = *(ADDRESS_SCB_AIRCR as *const u32);
    }

    (reg_val & SCB_AIR_PRIGROUP_MASK) >> SCB_AIR_PRIGROUP_POS
}

pub fn system_init() {
    // Set Reset state for RCC
    system_clock_set_default();

    // Set interrupt priority group
    nvic_set_priority_grouping(NVIC_PRIORITYGROUP_4);

    // Enable power clock
    reg_value_set_bit(RCC_APB1ENR as *mut u32, RCC_APB1ENR_PWREN_POS);
    // Set voltage scaling
    reg_value_set(PWR_CR1 as *mut u32, 0x0000C000);

    reg_value_set_bit(FLASH_ACR as *mut u32, FLASH_ACR_PRFTEN_POS);

    if VALUE_SVRANGE == VR_2700_3600 {
        if FOSC_KHZ_VALUE > 210_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_7WS);
        } else if FOSC_KHZ_VALUE > 180_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_6WS);
        } else if FOSC_KHZ_VALUE > 150_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_5WS);
        } else if FOSC_KHZ_VALUE > 120_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_4WS);
        } else if FOSC_KHZ_VALUE > 90_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_3WS);
        } else if FOSC_KHZ_VALUE > 60_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_2WS);
        } else if FOSC_KHZ_VALUE > 30_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_1WS);
        } else {
            reg_value_clear_mask(FLASH_ACR as *mut u32, !FLASH_ACR_LATENCY_MSK);
        }
    }
    if VALUE_SVRANGE == VR_2400_2700 {
        if FOSC_KHZ_VALUE > 192_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_8WS);
        } else if FOSC_KHZ_VALUE > 168_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_7WS);
        } else if FOSC_KHZ_VALUE > 144_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_6WS);
        } else if FOSC_KHZ_VALUE > 120_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_5WS);
        } else if FOSC_KHZ_VALUE > 96_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_4WS);
        } else if FOSC_KHZ_VALUE > 72_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_3WS);
        } else if FOSC_KHZ_VALUE > 48_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_2WS);
        } else if FOSC_KHZ_VALUE > 24_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_1WS);
        } else {
            reg_value_clear_mask(FLASH_ACR as *mut u32, !FLASH_ACR_LATENCY_MSK);
        }
    }
    if VALUE_SVRANGE == VR_2100_2400 {
        if FOSC_KHZ_VALUE > 198_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_9WS);
        } else if FOSC_KHZ_VALUE > 176_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_8WS);
        } else if FOSC_KHZ_VALUE > 154_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_7WS);
        } else if FOSC_KHZ_VALUE > 132_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_6WS);
        } else if FOSC_KHZ_VALUE > 110_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_5WS);
        } else if FOSC_KHZ_VALUE > 88_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_4WS);
        } else if FOSC_KHZ_VALUE > 66_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_3WS);
        } else if FOSC_KHZ_VALUE > 44_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_2WS);
        } else if FOSC_KHZ_VALUE > 22_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_1WS);
        } else {
            reg_value_clear_mask(FLASH_ACR as *mut u32, !FLASH_ACR_LATENCY_MSK);
        }
    }
    if VALUE_SVRANGE == VR_1800_2100 {
        if FOSC_KHZ_VALUE > 160_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_8WS);
        } else if FOSC_KHZ_VALUE > 140_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_7WS);
        } else if FOSC_KHZ_VALUE > 120_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_6WS);
        } else if FOSC_KHZ_VALUE > 100_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_5WS);
        } else if FOSC_KHZ_VALUE > 80_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_4WS);
        } else if FOSC_KHZ_VALUE > 60_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_3WS);
        } else if FOSC_KHZ_VALUE > 40_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_2WS);
        } else if FOSC_KHZ_VALUE > 20_000 {
            reg_value_set(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_1WS);
        } else {
            reg_value_clear_mask(FLASH_ACR as *mut u32, !FLASH_ACR_LATENCY_MSK);
        }
    }

    /* Enable Overdrive Mode. It can be enabled only if HSI or HSE clock is selected. */
    enable_overdrive_mode();

    reg_value_clear_set(RCC_PLLCFGR as *mut u32, VALUE_RCC_PLLCFGR);          /* set clock configuration register */
    reg_value_clear_set(RCC_CFGR as *mut u32, VALUE_RCC_CFGR);                /* set clock configuration register 2 */
    reg_value_clear_set(RCC_CR as *mut u32, VALUE_RCC_CR & 0x000FFFFF); /* do not start PLLs yet */

    if VALUE_RCC_CR & (1 << RCC_CR_HSION_POS) != 0 { /* if HSI enabled */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_HSIRDY_POS) == 0 {
            /* Wait for HSIRDY = 1 (HSI is ready) */
        }
    }      

    if VALUE_RCC_CR & (1 << RCC_CR_HSEON_POS) != 0 { /* if HSE enabled */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_HSERDY_POS) == 0 {
            /* Wait for HSERDY = 1 (HSE is ready) */
        }
    }

    if VALUE_RCC_CR & (1 << RCC_CR_PLLON_POS) != 0 { /* if PLL enabled */
        reg_value_set_bit(RCC_CR as *mut u32, RCC_CR_PLLON_POS); /* PLL On */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_PLLRDY_POS) == 0 {
            /* Wait for PLL1RDY = 1 (PLL is ready) */
        }
    }

    /* Wait till SYSCLK is stabilized (depending on selected clock) */
    while (reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_SWS_MSK) != ((VALUE_RCC_CFGR << 2) & RCC_CFGR_SWS_MSK) {
    }

    // FPU enabled by default by cortex_m_rt crate
}

// ==================== DELAYS ======================

fn get_clock_value(_clock : u32) -> u32
{
    _clock / 1000
}

#[inline(never)]
#[no_mangle]
#[link_section = ".ramfunc"]

pub fn delay_cyc(mut cycle_num : u32)
{    
    unsafe
    {
        asm!(
             
            "2:",
            "sub {0}, #1",
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
pub fn delay_us(time_us: u32) 
{
    /*
     * Delay for STM32F723ZE - default NECTO setup
     */
    delay_cyc(time_us * get_clock_value(FOSC_KHZ_VALUE) * 2 / 6 );
}

#[inline(never)]
pub fn delay_ms(time_ms: u32) 
{

    /*
     * Delay for STM32F723ZE - default NECTO setup
     */

    delay_us(time_ms * 1000);
}

#[inline(never)]
pub fn delay_advanced_ms(time_ms: u32, current_fosc_k_hz: u32) 
{

}

// Functions for specific delays in microseconds
pub fn delay_1us() {
    delay_us(1);
}

pub fn delay_5us() {
    delay_us(5);
}

pub fn delay_6us() {
    delay_us(6);
}

pub fn delay_9us() {
    delay_us(9);
}

pub fn delay_10us() {
    delay_us(10);
}

pub fn delay_22us() {
    delay_us(22);
}

pub fn delay_50us() {
    delay_us(50);
}

pub fn delay_55us() {
    delay_us(55);
}

pub fn delay_60us() {
    delay_us(60);
}

pub fn delay_64us() {
    delay_us(64);
}

pub fn delay_70us() {
    delay_us(70);
}

pub fn delay_80us() {
    delay_us(78);
}

pub fn delay_410us() {
    delay_us(410);
}

pub fn delay_480us() {
    delay_us(480);
}

pub fn delay_500us() {
    delay_us(498);
}

pub fn delay_5500us() {
    delay_us(5500);
}

// Functions for delays in milliseconds
pub fn delay_1ms() {
    delay_ms(1);
}

pub fn delay_5ms() {
    delay_ms(5);
}

pub fn delay_8ms() {
    delay_ms(8);
}

pub fn delay_10ms() {
    delay_ms(10);
}

pub fn delay_100ms() {
    delay_ms(100);
}

pub fn delay_1sec() {
    delay_ms(1000);
}