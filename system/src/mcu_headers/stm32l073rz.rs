pub const RCC_BASE       : u32 = 0x40021000;
pub const PWR_BASE       : u32 = 0x40007000;
pub const SYSCFG_BASE    : u32 = 0x40010000;

// =========== RCC REGISTERS ===========

pub const RCC_CR          : u32 = RCC_BASE;
pub const RCC_CRRCR       : u32 = RCC_BASE + 0x08;
pub const RCC_CFGR        : u32 = RCC_BASE + 0x0C;
pub const RCC_APB2ENR     : u32 = RCC_BASE + 0x34;

// =========== PWR REGISTERS ===========

pub const PWR_CR          : u32 = PWR_BASE;

// =========== SYSCFG REGISTERS ===========

pub const SYSCFG_CFGR3    : u32 = SYSCFG_BASE + 0x20;

// =====================================

/***** Bit definition for RCC_CR register  *****/

pub const RCC_CR_HSION_Pos       : u32   =  0;
pub const RCC_CR_HSEBYP_Pos      : u32   = 18;
pub const RCC_CR_HSIRDY_Pos      : u32   =  2;
pub const RCC_CR_HSEON_Pos       : u32   = 16;
pub const RCC_CR_HSERDY_Pos      : u32   = 17;
pub const RCC_CR_PLLON_Pos       : u32   = 24;
pub const RCC_CR_PLLRDY_Pos      : u32   = 25;
pub const RCC_CRRCR_HSI48RDY_Pos : u32   =  1;

pub const RCC_CRRCR_HSI48ON_Pos        : u32 = 0;
pub const RCC_APB2ENR_SYSCFGEN_Pos     : u32 = 0;
pub const SYSCFG_CFGR3_ENREF_HSI48_Pos : u32 = 13;

/***** Bit definition for RCC_CFGR register  *****/

/* SWS configuration */
pub const RCC_CFGR_SWS_Pos      : u32 = 2;
pub const RCC_CFGR_SWS_Msk      : u32 = 0x3 << RCC_CFGR_SWS_Pos;

/* HPRE configuration */
pub const RCC_CFGR_HPRE_Pos     : u32 = 4;
pub const RCC_CFGR_HPRE_Msk     : u32 = (0xF << RCC_CFGR_HPRE_Pos);

/* PPRE configuration */
pub const RCC_CFGR_PPRE_Pos     : u32 = 8;
pub const RCC_CFGR_PPRE_Msk     : u32 = (0x7 << RCC_CFGR_PPRE_Pos);

/* PPRE1 configuration */
pub const RCC_CFGR_PPRE1_Pos    : u32 = 8;
pub const RCC_CFGR_PPRE1_Msk    : u32 = (0x7 << RCC_CFGR_PPRE1_Pos);

/* PPRE2 configuration */
pub const RCC_CFGR_PPRE2_Pos    : u32 = 11;
pub const RCC_CFGR_PPRE2_Msk    : u32 = (0x7 << RCC_CFGR_PPRE2_Pos);

/* HSE Clock configuration */
pub const RCC_CR_CSSHSEON_Pos   : u32 = 19;
pub const RCC_CR_CSSHSEON_Msk   : u32 = (0x1 << RCC_CR_CSSHSEON_Pos);
pub const RCC_CR_CSSHSEON       : u32 = RCC_CR_CSSHSEON_Msk;

/*************************************************/

// =========== FLASH REGISTERS =========== 

pub const FLASH_R_BASE : u32 = 0x40022000;

pub const FLASH_ACR  : u32 = FLASH_R_BASE;

// =====================================

/***** Bit definition for FLASH_ACR register  *****/

pub const FLASH_ACR_LATENCY_Pos : u32 = 0;
pub const FLASH_ACR_LATENCY_Msk : u32 = (0x1 << FLASH_ACR_LATENCY_Pos);
pub const FLASH_ACR_LATENCY     : u32 = (FLASH_ACR_LATENCY_Msk);

pub const FLASH_ACR_PRFTEN_Pos  : u32 = 1;

/*************************************************/