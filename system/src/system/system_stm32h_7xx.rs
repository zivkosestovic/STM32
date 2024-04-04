use core::arch::asm;

use crate::common::common_header::*;
use crate::core_header::*;
use crate::mcu_headers::mcu::*;

const ADDRESS_SCB_AIRCR     : u32 = 0xE000ED0C;
const SCB_AIRCR_SYSRESETREQ : u32 = 2;

// Voltage range
// 1.15 to 1.26 V
const VR_1150_1260: u32 = 2;
// Voltage range
// 1.05 to 1.15 V
const VR_1050_1150: u32 = 1;
// Voltage range
// 0.95 to 1.05 V
const VR_0950_1050: u32 = 0;

pub struct RCC_ClocksTypeDef {
    pub SYSCLK_Frequency : u32,  // SYSCLK clock frequency in Hz
    pub HCLK_Frequency   : u32,  // HCLK clock frequency   in Hz
    pub PCLK1_Frequency  : u32,  // PCLK1 clock frequency  in Hz
    pub PCLK2_Frequency  : u32,  // PCLK2 clock frequency  in Hz
    pub PCLK3_Frequency  : u32,  // PCLK3 clock frequency  in Hz
    pub PCLK4_Frequency  : u32,  // PCLK4 clock frequency  in Hz
    pub ADCCLK_Frequency : u32,  // ADCCLK clock frequency in Hz
}

const APBPrescTable:    [u8; 8] = [1, 2, 3, 4, 6, 7, 8, 9];
const APB3_4PrescTable: [u8; 8] = [1, 0, 0, 0, 2, 4, 8, 16];


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

    /* Reset CFGR register */
    reg_value_clear(RCC_CFGR as *mut u32);

    /* Reset HSEON, CSSON , CSION,RC48ON, CSIKERON PLL1ON, PLL2ON and PLL3ON bits */
    reg_value_clear_mask(RCC_CR as *mut u32, 0xEAF6ED7F);

    /* Reset D1CFGR register */
    reg_value_clear(RCC_D1CFGR as *mut u32);

    /* Reset D2CFGR register */
    reg_value_clear(RCC_D2CFGR as *mut u32);

    /* Reset D3CFGR register */
    reg_value_clear(RCC_D3CFGR as *mut u32);

    /* Reset PLLCKSELR register */
    reg_value_clear(RCC_PLLCKSELR as *mut u32);

    /* Reset PLLCFGR register */
    reg_value_clear(RCC_PLLCFGR as *mut u32);

    /* Reset PLL1DIVR register */
    reg_value_set(RCC_PLL1DIVR as *mut u32, 0x00000200);

    /* Reset PLL1FRACR register */
    reg_value_clear(RCC_PLL1FRACR as *mut u32);

    /* Reset PLL2DIVR register */
    reg_value_set(RCC_PLL2DIVR as *mut u32, 0x00000200);

    /* Reset PLL2FRACR register */
    reg_value_clear(RCC_PLL2FRACR as *mut u32);

    /* Reset PLL3DIVR register */
    reg_value_set(RCC_PLL3DIVR as *mut u32, 0x00000200);

    /* Reset PLL3FRACR register */
    reg_value_clear(RCC_PLL3FRACR as *mut u32);

    /* Reset HSEBYP bit */
    reg_value_clear_bit(RCC_CR as *mut u32, RCC_CR_HSEBYP_Pos);

    /* Disable all interrupts */
    reg_value_clear(RCC_CICR as *mut u32);
}

pub fn rcc_get_clocks_frequency(rcc_clocks: &mut RCC_ClocksTypeDef) {
    let mut tmp: u32;
    let mut presc1: u8;
    let mut presc2: u8;

    /* Get System clock */
    rcc_clocks.SYSCLK_Frequency = FOSC_KHZ_VALUE * 1000;

    /* Get HCLK prescaler */
    if (reg_value_get(RCC_D1CFGR as *mut u32) & 0x08) != 0 {
        let tmp = reg_value_get(RCC_D1CFGR as *mut u32) & 0x07;
        presc1 = APBPrescTable[tmp as usize];
    } else {
        presc1 = 0;
    }

    /* Get D1C prescaler */
    if (reg_value_get(RCC_D1CFGR as *mut u32) & 0x0800) != 0 {
        let tmp = (reg_value_get(RCC_D1CFGR as *mut u32) & 0x0700) >> 8;
        presc2 = APBPrescTable[tmp as usize];
    } else {
        presc2 = 0;
    }
    
    /* HCLK clock frequency */
    rcc_clocks.HCLK_Frequency = rcc_clocks.SYSCLK_Frequency >> (presc1 + presc2);

    /* Get PCLK1 prescaler */
    if (reg_value_get(RCC_D2CFGR as *mut u32) & 0x40) != 0 {
        let tmp = (reg_value_get(RCC_D2CFGR as *mut u32) & 0x30) >> 4;
        presc1 = APBPrescTable[tmp as usize];
    } else {
        presc1 = 0;
    }

    /* PCLK1 clock frequency */
    rcc_clocks.PCLK1_Frequency = rcc_clocks.HCLK_Frequency >> presc1;

    /* Get PCLK2 prescaler */
    if(reg_value_get(RCC_D2CFGR as *mut u32) & 0x0E00) != 0 {
        tmp = (reg_value_get(RCC_D2CFGR as *mut u32) & 0x06) >> 8;
        presc1 = APBPrescTable[tmp as usize];
    } else {
        presc1 = 0;
    }

    /* PCLK2 clock frequency */
    rcc_clocks.PCLK2_Frequency = rcc_clocks.HCLK_Frequency >> presc1;

    /* Get PCLK3 prescaler */
    if(reg_value_get(RCC_D1CFGR as *mut u32) & 0x40) != 0 {
        tmp = ( reg_value_get(RCC_D1CFGR as *mut u32) & 0x70 ) >> 4;
        presc1 = APB3_4PrescTable[tmp as usize];
    } else {
        presc1 = APB3_4PrescTable[0];
    }
        
    /* PCLK3 clock frequency */
    rcc_clocks.PCLK3_Frequency = ( rcc_clocks.HCLK_Frequency ) / presc1 as u32;

    /* PCLK4 clock frequency */
    /* Always has the same value as PCLK3 */
    rcc_clocks.PCLK4_Frequency = rcc_clocks.PCLK3_Frequency;
}

pub fn system_init() {
    system_clock_set_default();

    // Enable power clock
    reg_value_set_bit(RCC_APB1HENR as *mut u32, RCC_APB1HENR_OPAMPEN_Pos);

    let mut voltage_range: u32 = VR_1150_1260;

    // Set VCORE
    if VALUE_PWR_D3CR == 0x0000C000 {
        voltage_range = VR_1150_1260;
    } else if VALUE_PWR_D3CR == 0x00008000 {
        voltage_range = VR_1050_1150;
    } else if VALUE_PWR_D3CR == 0x00004000 {
        voltage_range = VR_0950_1050;
    }

    reg_value_set(PWR_D3CR as *mut u32, VALUE_PWR_D3CR); // Internal VCORE voltage

    if voltage_range == VR_1150_1260 {
        if FOSC_KHZ_VALUE > 225000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_4WS | FLASH_ACR_WRHIGHFREQ_1); // latency = 4, WRHIGHFREQ = 0b10
        }
        else if FOSC_KHZ_VALUE > 210000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_3WS | FLASH_ACR_WRHIGHFREQ_1); // latency = 3, WRHIGHFREQ = 0b10
        }
        else if FOSC_KHZ_VALUE > 185000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_2WS | FLASH_ACR_WRHIGHFREQ_1); // latency = 2, WRHIGHFREQ = 0b10
        }
        else if FOSC_KHZ_VALUE > 140000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_2WS | FLASH_ACR_WRHIGHFREQ_0); // latency = 2, WRHIGHFREQ = 0b01
        }
        else if FOSC_KHZ_VALUE > 70000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_1WS | FLASH_ACR_WRHIGHFREQ_0); // latency = 1, WRHIGHFREQ = 0b01
        }
        else {
            reg_value_clear_mask(FLASH_ACR as *mut u32, 0);    // latency = 0, WRHIGHFREQ = 0b00
        }
    } 
    else if voltage_range == VR_1050_1150 {
        if FOSC_KHZ_VALUE >= 225000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_4WS | FLASH_ACR_WRHIGHFREQ_1); // latency = 4, WRHIGHFREQ = 0b10
        }
        else if FOSC_KHZ_VALUE > 165000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_3WS | FLASH_ACR_WRHIGHFREQ_1); // latency = 3, WRHIGHFREQ = 0b10
        }
        else if FOSC_KHZ_VALUE > 110000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_2WS | FLASH_ACR_WRHIGHFREQ_0); // latency = 2, WRHIGHFREQ = 0b01
        }
        else if FOSC_KHZ_VALUE > 55000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_1WS | FLASH_ACR_WRHIGHFREQ_0); // latency = 1, WRHIGHFREQ = 0b01
        }
        else {
            reg_value_clear_mask(FLASH_ACR as *mut u32, 0);    // latency = 0, WRHIGHFREQ = 0b00
        }
    }
    else if voltage_range == VR_0950_1050 {
        if FOSC_KHZ_VALUE > 225000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_4WS | FLASH_ACR_WRHIGHFREQ_1); //latency = 4, WRHIGHFREQ = 0b10
        }
        else if FOSC_KHZ_VALUE > 180000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_4WS | FLASH_ACR_WRHIGHFREQ_1); //latency = 4, WRHIGHFREQ = 0b10
        }
        else if FOSC_KHZ_VALUE > 135000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_3WS | FLASH_ACR_WRHIGHFREQ_1); // latency = 3, WRHIGHFREQ = 0b10
        }
        else if FOSC_KHZ_VALUE > 90000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_2WS | FLASH_ACR_WRHIGHFREQ_0); // latency = 2, WRHIGHFREQ = 0b01
        }
        else if FOSC_KHZ_VALUE > 45000 {
            reg_value_clear_mask(FLASH_ACR as *mut u32, FLASH_ACR_LATENCY_1WS | FLASH_ACR_WRHIGHFREQ_0); // latency = 1, WRHIGHFREQ = 0b01
        }
        else {
            reg_value_clear_mask(FLASH_ACR as *mut u32, 0);    // latency = 0, WRHIGHFREQ = 0b00
        }
    }

    reg_value_set(RCC_CFGR as *mut u32,            VALUE_RCC_CFGR);                 /* set clock configuration register 2 */
    reg_value_set(RCC_CR as *mut u32,              VALUE_RCC_CR & 0x000FF3BF);      /* do not start PLLs yet */
    reg_value_set(RCC_PLLCKSELR as *mut u32,       VALUE_RCC_PLLCKSELR);            /* set PLL prescaler register */
    reg_value_set(RCC_PLLCFGR as *mut u32,         VALUE_RCC_PLLCFGR);              /* set clock configuration register */
    reg_value_set(RCC_CSR as *mut u32,             VALUE_RCC_CSR);                  /* set LSI clock */
    reg_value_set(RCC_BDCR as *mut u32,            VALUE_RCC_BDCR);                 /* set LSE clock */
    reg_value_set(RCC_D1CFGR as *mut u32,          VALUE_RCC_D1CFGR);               /* set clock domain 1 */
    reg_value_set(RCC_D2CFGR as *mut u32,          VALUE_RCC_D2CFGR);               /* set clock domain 2 */
    reg_value_set(RCC_D3CFGR as *mut u32,          VALUE_RCC_D3CFGR);               /* set clock domain 3 */
    reg_value_set(RCC_PLL1DIVR as *mut u32,        VALUE_RCC_PLL1DIVR);             /* set PLL1 configuration register */
    reg_value_set(RCC_PLL2DIVR as *mut u32,        VALUE_RCC_PLL2DIVR);             /* set PLL2 configuration register */
    reg_value_set(RCC_PLL3DIVR as *mut u32,        VALUE_RCC_PLL3DIVR);             /* set PLL3 configuration register */

    if VALUE_RCC_CR & (1 << RCC_CR_HSION_Pos) != 0 { /* if HSI enabled */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_HSIRDY_Pos) == 0 {
            /* Wait for HSIRDY = 1 (HSI is ready) */
        }
    }      

    if VALUE_RCC_CR & (1 << RCC_CR_HSEON_Pos) != 0 { /* if HSE enabled */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_HSERDY_Pos) == 0 {
            /* Wait for HSERDY = 1 (HSE is ready) */
        }
    }

    if VALUE_RCC_CR & (1 << RCC_CR_PLL1ON_Pos) != 0 { /* if PLL1 enabled */
        reg_value_set_bit(RCC_CR as *mut u32, RCC_CR_PLL1ON_Pos); /* PLL1 On */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_PLL1RDY_Pos) == 0 {
            /* Wait for PLL1RDY = 1 (PLL is ready) */
        }
    }

    if VALUE_RCC_CR & (1 << RCC_CR_PLL2ON_Pos) != 0 { /* if PLL2 enabled */
        reg_value_set_bit(RCC_CR as *mut u32, RCC_CR_PLL2ON_Pos); /* PLL2 On */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_PLL2RDY_Pos) == 0 {
            /* Wait for PLL2RDY = 1 (PLL is ready) */
        }
    }

    if VALUE_RCC_CR & (1 << RCC_CR_PLL3ON_Pos) != 0 { /* if PLL3 enabled */
        reg_value_set_bit(RCC_CR as *mut u32, RCC_CR_PLL3ON_Pos); /* PLL3 On */
        while reg_value_get_bit(RCC_CR as *mut u32, RCC_CR_PLL3RDY_Pos) == 0 {
            /* Wait for PLL3RDY = 1 (PLL is ready) */
        }
    }

    /* Wait till SYSCLK is stabilized (depending on selected clock) */    
    while (reg_value_get(RCC_CFGR as *mut u32) & RCC_CFGR_SWS_Pos) != ((VALUE_RCC_CFGR << 3) & RCC_CFGR_SWS_Pos) {
    }

    // FPU enabled by default by cortex-m-crate
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
pub fn Delay_us(time_us: u32) 
{
    /*
     * Delay for STM32H753ZI - default NECTO setup
     */
    Delay_Cyc(time_us * get_clock_value(FOSC_KHZ_VALUE) / 3 );
}


#[inline(never)]
pub fn Delay_ms(time_ms: u32) 
{

    /*
     * Delay for STM32H753ZI - default NECTO setup
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