/*____ Core Header for STM32H753ZI 8 Mhz, generated by NECTO ___*/

pub const ADDRESS_RCC_CR  : u32 = 0x58024400;
pub const VALUE_RCC_CR  : u32 = 0x19;
pub const ADDRESS_RCC_CFGR  : u32 = 0x58024410;
pub const VALUE_RCC_CFGR  : u32 = 0x0;
pub const ADDRESS_RCC_D1CFGR  : u32 = 0x58024418;
pub const VALUE_RCC_D1CFGR  : u32 = 0x0;
pub const ADDRESS_RCC_D2CFGR  : u32 = 0x5802441C;
pub const VALUE_RCC_D2CFGR  : u32 = 0x0;
pub const ADDRESS_RCC_D3CFGR  : u32 = 0x58024420;
pub const VALUE_RCC_D3CFGR  : u32 = 0x0;
pub const ADDRESS_RCC_PLLCKSELR : u32 = 0x58024428;
pub const VALUE_RCC_PLLCKSELR  : u32 = 0x2020200;
pub const ADDRESS_RCC_PLLCFGR  : u32 = 0x5802442C;
pub const VALUE_RCC_PLLCFGR  : u32 = 0x0;
pub const ADDRESS_RCC_PLL1DIVR  : u32 = 0x58024430;
pub const VALUE_RCC_PLL1DIVR  : u32 = 0x1010480;
pub const ADDRESS_RCC_PLL2DIVR  : u32 = 0x58024438;
pub const VALUE_RCC_PLL2DIVR  : u32 = 0x1010480;
pub const ADDRESS_RCC_PLL3DIVR  : u32 = 0x58024440;
pub const VALUE_RCC_PLL3DIVR  : u32 = 0x1010480;
pub const ADDRESS_RCC_BDCR  : u32 = 0x58024470;
pub const VALUE_RCC_BDCR  : u32 = 0x0;
pub const ADDRESS_RCC_CSR  : u32 = 0x58024474;
pub const VALUE_RCC_CSR  : u32 = 0x0;
pub const ADDRESS_PWR_D3CR  : u32 = 0x58024818;
pub const VALUE_PWR_D3CR  : u32 = 0xC000;
pub const FOSC_KHZ_VALUE  : u32 = 8000;

/*____ Core Header for STM32H753ZI 480 Mhz  * DOESNT WORK * ___*/

// pub const ADDRESS_RCC_CR: u32 = 0x58024400;
// pub const VALUE_RCC_CR: u32 = 0x1000001;  // HSI ON
// pub const ADDRESS_RCC_CFGR: u32 = 0x58024410;
// pub const VALUE_RCC_CFGR: u32 = 0x3;      // HSI as system clock
// pub const ADDRESS_RCC_D1CFGR: u32 = 0x58024418;
// pub const VALUE_RCC_D1CFGR: u32 = 0x1;    // D1CPRE = 1 (divide by 1)
// pub const ADDRESS_RCC_D2CFGR: u32 = 0x5802441C;
// pub const VALUE_RCC_D2CFGR: u32 = 0x800;  // HPRE = 2 (divide by 2)
// pub const ADDRESS_RCC_D3CFGR: u32 = 0x58024420;
// pub const VALUE_RCC_D3CFGR: u32 = 0x100;  // D1PPRE/D2PPRE1/D2PPRE2/D3PPRE = 2 (divide by 2)
// pub const ADDRESS_RCC_PLLCKSELR: u32 = 0x58024428;
// pub const VALUE_RCC_PLLCKSELR: u32 = 0x01000000;  // HSI as PLL input, PLLM = 4
// pub const ADDRESS_RCC_PLLCFGR: u32 = 0x5802442C;
// pub const VALUE_RCC_PLLCFGR: u32 = 0x00060000;  // PLLN = 60, PLLP = 2
// pub const ADDRESS_RCC_PLL1DIVR: u32 = 0x58024430;
// pub const VALUE_RCC_PLL1DIVR: u32 = 0x1010000;  // PLL1P = 2
// pub const ADDRESS_RCC_PLL2DIVR: u32 = 0x58024438;
// pub const VALUE_RCC_PLL2DIVR: u32 = 0x1010000;  // PLL2P = 2
// pub const ADDRESS_RCC_PLL3DIVR: u32 = 0x58024440;
// pub const VALUE_RCC_PLL3DIVR: u32 = 0x1010000;  // PLL3P = 2
// pub const ADDRESS_RCC_BDCR: u32 = 0x58024470;
// pub const VALUE_RCC_BDCR: u32 = 0x0;
// pub const ADDRESS_RCC_CSR: u32 = 0x58024474;
// pub const VALUE_RCC_CSR: u32 = 0x0;
// pub const ADDRESS_PWR_D3CR: u32 = 0x58024818;
// pub const VALUE_PWR_D3CR: u32 = 0xC000;
// pub const FOSC_KHZ_VALUE: u32 = 480000;