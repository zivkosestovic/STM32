pub const RCC_BASE        : u32 = 0x40021000;
pub const PWR_BASE        : u32 = 0x40007000;

// =========== RCC REGISTERS ===========

pub const RCC_CR          : u32 = RCC_BASE;
pub const RCC_CFGR        : u32 = RCC_BASE + 0x08;
pub const RCC_PLLCFGR     : u32 = RCC_BASE + 0x0C;
pub const RCC_BDCR        : u32 = RCC_BASE + 0x5C;
pub const RCC_CSR         : u32 = RCC_BASE + 0x60;
pub const RCC_CCIPR       : u32 = RCC_BASE + 0x54;
pub const RCC_APBENR1     : u32 = RCC_BASE + 0x3C;
pub const RCC_APBENR2     : u32 = RCC_BASE + 0x40;

// =========== PWR REGISTERS ===========

pub const PWR_CR1        : u32 = PWR_BASE;
pub const PWR_SR2        : u32 = PWR_BASE + 0x14;

// =====================================

/***** Bit definition for RCC_CR register  *****/

pub const RCC_CR_HSION_Pos       : u32   =  8;
pub const RCC_CR_HSIRDY_Pos      : u32   = 10;
pub const RCC_CR_HSEON_Pos       : u32   = 16;
pub const RCC_CR_HSERDY_Pos      : u32   = 17;
pub const RCC_CR_PLLON_Pos       : u32   = 24;
pub const RCC_CR_PLLRDY_Pos      : u32   = 25;

pub const RCC_CSR_LSION_Pos      : u32 = 0;
pub const RCC_CSR_LSIRDY_Pos     : u32 = 1;
pub const RCC_BDCR_LSEON_Pos     : u32 = 0;
pub const RCC_BDCR_LSERDY_Pos    : u32 = 1;
    
/***** Bit definition for RCC_CFGR register  *****/

/* SWS configuration */
pub const RCC_CFGR_SWS_Pos      : u32 = 3;
pub const RCC_CFGR_SWS_Msk      : u32 = 0x7 << RCC_CFGR_SWS_Pos;

/* HPRE configuration */
pub const RCC_CFGR_HPRE_Pos     : u32 = 8;  
pub const RCC_CFGR_HPRE_Msk     : u32 = (0xF << RCC_CFGR_HPRE_Pos);

/* PPRE configuration */
pub const RCC_CFGR_PPRE_Pos     : u32 = 12;
pub const RCC_CFGR_PPRE_Msk     : u32 = (0x7 << RCC_CFGR_PPRE_Pos);

pub const RCC_PLLCFGR_PLLSRC_Pos   : u32 = 0;
pub const RCC_PLLCFGR_PLLSRC_Msk   : u32 = (0x3 << RCC_PLLCFGR_PLLSRC_Pos);

pub const RCC_APBENR1_PWREN_Pos    : u32 = 28;
pub const RCC_APBENR2_SYSCFGEN_Pos : u32 = 0;

pub const PWR_SR2_VOSF_Pos         : u32 = 10;

/*************************************************/

// =========== FLASH REGISTERS =========== 

pub const FLASH_R_BASE : u32 = 0x40022000;

pub const FLASH_ACR    : u32 = FLASH_R_BASE;

// =====================================

/***** Bit definition for FLASH_ACR register  *****/

pub const FLASH_ACR_LATENCY_Pos : u32 = 0;
pub const FLASH_ACR_LATENCY_Msk : u32 = (0x7 << FLASH_ACR_LATENCY_Pos);
pub const FLASH_ACR_LATENCY     : u32 = (FLASH_ACR_LATENCY_Msk);
pub const FLASH_ACR_LATENCY_0   : u32 = (0x1 << FLASH_ACR_LATENCY_Pos);
pub const FLASH_ACR_LATENCY_1   : u32 = (0x2 << FLASH_ACR_LATENCY_Pos);
pub const FLASH_ACR_LATENCY_2   : u32 = (0x4 << FLASH_ACR_LATENCY_Pos);

pub const FLASH_ACR_PRFTEN_Pos  : u32 = 8;

/***** Bit definition for PWR_CR register  *****/

pub const PWR_CR1_VOS_Pos       : u32 = 9;
pub const PWR_CR1_VOS_Msk       : u32 = (0x3 << PWR_CR1_VOS_Pos);
pub const PWR_CR1_VOS           : u32 = PWR_CR1_VOS_Msk;
pub const PWR_CR1_VOS_0         : u32 = (0x1 << PWR_CR1_VOS_Pos);
pub const PWR_CR1_VOS_1         : u32 = (0x2 << PWR_CR1_VOS_Pos);