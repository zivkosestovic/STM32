pub const RCC_BASE : u32  = 0x40023800;
pub const PWR_BASE : u32  = 0x40007000;

/* =========== RCC REGISTERS =========== */

pub const RCC_CR      : u32 = RCC_BASE;
pub const RCC_PLLCFGR : u32 = RCC_BASE + 0x04;
pub const RCC_CFGR    : u32 = RCC_BASE + 0x08;
pub const RCC_CIR     : u32 = RCC_BASE + 0x0C;
pub const RCC_APB1ENR : u32 = RCC_BASE + 0x40;

/* =========== PWR REGISTERS =========== */

pub const PWR_CR1  : u32 = PWR_BASE;
pub const PWR_CSR1 : u32 = PWR_BASE + 0x04;

// =====================================

/***** Bit definition for RCC_CR register  *****/

pub const RCC_CR_HSION_POS  : u32   =  0;
pub const RCC_CR_HSEBYP_POS : u32   = 18;
pub const RCC_CR_HSIRDY_POS : u32   =  1;
pub const RCC_CR_HSEON_POS  : u32   = 16;
pub const RCC_CR_HSERDY_POS : u32   = 17;
pub const RCC_CR_PLLON_POS  : u32   = 24;
pub const RCC_CR_PLLRDY_POS : u32   = 25;

/***** Bit definition for PWR_CR register  *****/

pub const RCC_APB1ENR_PWREN_POS : u32 = 28;

pub const PWR_CR1_ODEN_POS      : u32 = 16;
pub const PWR_CSR1_ODRDY_POS    : u32 = 16;

pub const PWR_CR1_ODSWEN_POS    : u32 = 17;
pub const PWR_CSR1_ODSWRDY_POS  : u32 = 17;

/***** Bit definition for RCC_CFGR register  *****/

/* SWS configuration */
pub const RCC_CFGR_SWS_POS      : u32 = 2;
pub const RCC_CFGR_SWS_MSK      : u32 = 0x3 << RCC_CFGR_SWS_POS;

/* HPRE configuration */
pub const RCC_CFGR_HPRE_POS     : u32 = 4;  
pub const RCC_CFGR_HPRE_MSK     : u32 = 0xF << RCC_CFGR_HPRE_POS;

/* PPRE1 configuration */
pub const RCC_CFGR_PPRE1_POS    : u32 = 10;                              
pub const RCC_CFGR_PPRE1_MSK    : u32 = 0x7 << RCC_CFGR_PPRE1_POS;

/* PPRE2 configuration */
pub const RCC_CFGR_PPRE2_POS    : u32 = 13;                            
pub const RCC_CFGR_PPRE2_MSK    : u32 = 0x7 << RCC_CFGR_PPRE2_POS;

/*************************************************/

// =========== FLASH REGISTERS =========== 

pub const FLASH_R_BASE          : u32 = 0x40023C00;

pub const FLASH_ACR             : u32 = FLASH_R_BASE + 0x00;

// =====================================

/***** Bit definition for FLASH_ACR register  *****/

pub const FLASH_ACR_PRFTEN_POS  : u32 = 8;

pub const FLASH_ACR_LATENCY_POS : u32 = 0;
pub const FLASH_ACR_LATENCY_MSK : u32 = 0xF << FLASH_ACR_LATENCY_POS;

pub const FLASH_ACR_LATENCY_1WS : u32 = 0x00000001;
pub const FLASH_ACR_LATENCY_2WS : u32 = 0x00000002;
pub const FLASH_ACR_LATENCY_3WS : u32 = 0x00000003;
pub const FLASH_ACR_LATENCY_4WS : u32 = 0x00000004;
pub const FLASH_ACR_LATENCY_5WS : u32 = 0x00000005;
pub const FLASH_ACR_LATENCY_6WS : u32 = 0x00000006;
pub const FLASH_ACR_LATENCY_7WS : u32 = 0x00000007;
pub const FLASH_ACR_LATENCY_8WS : u32 = 0x00000008;
pub const FLASH_ACR_LATENCY_9WS : u32 = 0x00000009;

/*************************************************/