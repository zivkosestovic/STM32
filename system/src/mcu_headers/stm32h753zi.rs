pub const RCC_BASE        : u32 = 0x58024400;
pub const PWR_BASE        : u32 = 0x58024800;

// =========== RCC REGISTERS ===========

pub const RCC_CR          : u32 = RCC_BASE;
pub const RCC_CFGR        : u32 = RCC_BASE + 0x10;
pub const RCC_PLLCFGR     : u32 = RCC_BASE + 0x2C;
pub const RCC_D1CFGR      : u32 = RCC_BASE + 0x18;
pub const RCC_D2CFGR      : u32 = RCC_BASE + 0x1C;
pub const RCC_D3CFGR      : u32 = RCC_BASE + 0x20;
pub const RCC_PLLCKSELR   : u32 = RCC_BASE + 0x28;
pub const RCC_PLL1DIVR    : u32 = RCC_BASE + 0x30;
pub const RCC_PLL2DIVR    : u32 = RCC_BASE + 0x38;
pub const RCC_PLL1FRACR   : u32 = RCC_BASE + 0x34;
pub const RCC_PLL2FRACR   : u32 = RCC_BASE + 0x3C;
pub const RCC_PLL3DIVR    : u32 = RCC_BASE + 0x40;
pub const RCC_PLL3FRACR   : u32 = RCC_BASE + 0x44;
pub const RCC_CICR        : u32 = RCC_BASE + 0x68;
pub const RCC_BDCR        : u32 = RCC_BASE + 0x70;
pub const RCC_CSR         : u32 = RCC_BASE + 0x74;
pub const RCC_APB1HENR    : u32 = RCC_BASE + 0xEC;

// =========== PWR REGISTERS ===========

pub const PWR_D3CR        : u32 = PWR_BASE + 0x18;

// =====================================

/***** Bit definition for RCC_CR register  *****/

pub const RCC_CR_HSION_Pos       : u32 =  0;
pub const RCC_CR_HSEBYP_Pos      : u32 = 18;
pub const RCC_CR_HSIRDY_Pos      : u32 =  2;
pub const RCC_CR_HSEON_Pos       : u32 = 16;
pub const RCC_CR_HSERDY_Pos      : u32 = 17;
pub const RCC_CR_PLLON_Pos       : u32 = 24;
pub const RCC_CR_PLLRDY_Pos      : u32 = 25;

pub const RCC_CR_PLL1ON_Pos      : u32 = 24;
pub const RCC_CR_PLL1RDY_Pos     : u32 = 25;
pub const RCC_CR_PLL2ON_Pos      : u32 = 26;
pub const RCC_CR_PLL2RDY_Pos     : u32 = 27;
pub const RCC_CR_PLL3ON_Pos      : u32 = 28;
pub const RCC_CR_PLL3RDY_Pos     : u32 = 29;

pub const RCC_CSR_LSION_Pos      : u32 =  0;
pub const RCC_CSR_LSIRDY_Pos     : u32 =  1;
pub const RCC_BDCR_LSEON_Pos     : u32 =  0;
pub const RCC_BDCR_LSERDY_Pos    : u32 =  1;
    
/***** Bit definition for RCC_CFGR register  *****/

pub const RCC_CFGR_SWS_Pos      : u32 = 3;
pub const RCC_CFGR_SWS_Msk      : u32 = (0x7 << RCC_CFGR_SWS_Pos);
pub const RCC_CFGR_SWS          : u32 = RCC_CFGR_SWS_Msk;      

pub const RCC_APB1HENR_OPAMPEN_Pos : u32 = 4;

// =========== FLASH REGISTERS =========== 

pub const FLASH_R_BASE : u32 = 0x52002000;

pub const FLASH_ACR    : u32 = FLASH_R_BASE;

// =====================================

/***** Bit definition for FLASH_ACR register  *****/

pub const FLASH_ACR_LATENCY_1WS : u32 = 0x00000001;
pub const FLASH_ACR_LATENCY_2WS : u32 = 0x00000002;
pub const FLASH_ACR_LATENCY_3WS : u32 = 0x00000003;
pub const FLASH_ACR_LATENCY_4WS : u32 = 0x00000004;
pub const FLASH_ACR_LATENCY_5WS : u32 = 0x00000005;

pub const FLASH_ACR_WRHIGHFREQ_Pos : u32 = 4;
pub const FLASH_ACR_WRHIGHFREQ_Msk : u32 = (0x3 << FLASH_ACR_WRHIGHFREQ_Pos);
pub const FLASH_ACR_WRHIGHFREQ     : u32 = FLASH_ACR_WRHIGHFREQ_Msk;
pub const FLASH_ACR_WRHIGHFREQ_0   : u32 = (0x1 << FLASH_ACR_WRHIGHFREQ_Pos);
pub const FLASH_ACR_WRHIGHFREQ_1   : u32 = (0x2 << FLASH_ACR_WRHIGHFREQ_Pos);

/*************************************************/