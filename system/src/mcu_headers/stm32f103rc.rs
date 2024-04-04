pub const RCC_BASE        : u32 = 0x40021000;

// =========== RCC REGISTERS =========== 

pub const RCC_CR          : u32 = RCC_BASE;   
pub const RCC_CFGR        : u32 = RCC_BASE + 0x04;
pub const RCC_CIR         : u32 = RCC_BASE + 0x08;
pub const RCC_APB2RSTR    : u32 = RCC_BASE + 0x0C;
pub const RCC_APB1RSTR    : u32 = RCC_BASE + 0x10;
pub const RCC_AHBENR      : u32 = RCC_BASE + 0x14;
pub const RCC_APB2ENR     : u32 = RCC_BASE + 0x18;
pub const RCC_APB1ENR     : u32 = RCC_BASE + 0x1C;
pub const RCC_BDCR        : u32 = RCC_BASE + 0x20;
pub const RCC_CSR         : u32 = RCC_BASE + 0x24;

// =====================================

/***** Bit definition for RCC_CR register  *****/

pub const RCC_CR_HSION_Pos  : u32   = 0;
pub const RCC_CR_HSEBYP_Pos : u32   = 18;
pub const RCC_CR_HSIRDY_Pos : u32   = 1;
pub const RCC_CR_HSEON_Pos  : u32   = 16;
pub const RCC_CR_HSERDY_Pos : u32   = 17;
pub const RCC_CR_PLLON_Pos  : u32   = 24;   
pub const RCC_CR_PLLRDY_Pos : u32   = 25;                             

/***** Bit definition for RCC_CFGR register  *****/

/* SWS configuration */
pub const RCC_CFGR_SWS_Pos    : u32 = 2;
pub const RCC_CFGR_SWS_Msk    : u32 = 0x3 << RCC_CFGR_SWS_Pos;

/* HPRE configuration */
pub const RCC_CFGR_HPRE_Pos   : u32 = 4;  
pub const RCC_CFGR_HPRE_Msk   : u32 = (0xF << RCC_CFGR_HPRE_Pos);

/* PPRE1 configuration */
pub const RCC_CFGR_PPRE1_Pos  : u32 = 8;                              
pub const RCC_CFGR_PPRE1_Msk  : u32 = (0x7 << RCC_CFGR_PPRE1_Pos);

/* PPRE2 configuration */
pub const RCC_CFGR_PPRE2_Pos  : u32 = 11;                            
pub const RCC_CFGR_PPRE2_Msk  : u32 = (0x7 << RCC_CFGR_PPRE2_Pos);

/* ADCPRE configuration */
pub const RCC_CFGR_ADCPRE_Pos : u32 = 14;                             
pub const RCC_CFGR_ADCPRE_Msk : u32 = (0x3 << RCC_CFGR_ADCPRE_Pos);

/*************************************************/

// =========== FLASH REGISTERS =========== 

pub const FLASH_BASE      : u32 = 0x40022000;

pub const FLASH_ACR       : u32 = FLASH_BASE;
pub const FLASH_KEYR      : u32 = FLASH_BASE + 0x04;
pub const FLASH_OPTKEYR   : u32 = FLASH_BASE + 0x08;
pub const FLASH_SR        : u32 = FLASH_BASE + 0x0C;
pub const FLASH_CR        : u32 = FLASH_BASE + 0x10;
pub const FLASH_AR        : u32 = FLASH_BASE + 0x14;
pub const FLASH_RESERVED  : u32 = FLASH_BASE + 0x18;
pub const FLASH_OBR       : u32 = FLASH_BASE + 0x1C;
pub const FLASH_WRPR      : u32 = FLASH_BASE + 0x20;

// =====================================

/***** Bit definition for FLASH_ACR register  *****/

pub const FLASH_ACR_LATENCY_Pos : u32 = 0;
pub const FLASH_ACR_LATENCY_Msk : u32 = (0x7 << FLASH_ACR_LATENCY_Pos);
pub const FLASH_ACR_LATENCY_0   : u32 = (0x1 << FLASH_ACR_LATENCY_Pos);
pub const FLASH_ACR_LATENCY_1   : u32 = (0x2 << FLASH_ACR_LATENCY_Pos);

/*************************************************/