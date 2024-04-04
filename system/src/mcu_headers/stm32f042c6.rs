pub const RCC_BASE        : u32 = 0x40021000;

// =========== RCC REGISTERS ===========

pub const RCC_CR          : u32 = RCC_BASE;
pub const RCC_CR2         : u32 = RCC_BASE + 0x34;
pub const RCC_CFGR        : u32 = RCC_BASE + 0x04;
pub const RCC_CFGR2       : u32 = RCC_BASE + 0x2C;
pub const RCC_CFGR3       : u32 = RCC_BASE + 0x30;
pub const RCC_CSR         : u32 = RCC_BASE + 0x24;

// =====================================

/***** Bit definition for RCC_CR register  *****/

pub const RCC_CR_HSION_Pos   : u32   =  0;
pub const RCC_CR_HSEBYP_Pos  : u32   = 18;
pub const RCC_CR_HSIRDY_Pos  : u32   =  1;
pub const RCC_CR_HSEON_Pos   : u32   = 16;
pub const RCC_CR_HSERDY_Pos  : u32   = 17;
pub const RCC_CR_PLLON_Pos   : u32   = 24;
pub const RCC_CR_PLLRDY_Pos  : u32   = 25;

/***** Bit definition for RCC_CFGR register  *****/

/* SWS configuration */
pub const RCC_CFGR_SWS_Pos      : u32 = 2;
pub const RCC_CFGR_SWS_Msk      : u32 = 0x3 << RCC_CFGR_SWS_Pos;

/* HPRE configuration */
pub const RCC_CFGR_HPRE_Pos     : u32 = 4;  
pub const RCC_CFGR_HPRE_Msk     : u32 = (0xF << RCC_CFGR_HPRE_Pos);

pub const RCC_CFGR2_PREDIV_Pos  : u32 = 0;
pub const RCC_CFGR2_PREDIV_Msk  : u32 = (0xF << RCC_CFGR2_PREDIV_Pos);

pub const RCC_CR2_HSI14ON_Pos   : u32 = 0;

pub const RCC_CFGR_PPRE_Pos     : u32 = 8;
pub const RCC_CFGR_PPRE_Msk     : u32 = (0x7 << RCC_CFGR_PPRE_Pos);

/*************************************************/

// =========== FLASH REGISTERS =========== 

pub const FLASH_R_BASE   : u32 = 0x40022000;

pub const FLASH_ACR      : u32 = FLASH_R_BASE;

// =====================================

/***** Bit definition for FLASH_ACR register  *****/

pub const FLASH_ACR_LATENCY_Pos : u32 = 0;
pub const FLASH_ACR_LATENCY_Msk : u32 = (0x1 << FLASH_ACR_LATENCY_Pos);
pub const FLASH_ACR_LATENCY     : u32 = (FLASH_ACR_LATENCY_Msk);

/*************************************************/