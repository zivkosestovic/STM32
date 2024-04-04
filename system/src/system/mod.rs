/* ====== STM32F042C6 ====== */

#[cfg(feature = "stm32f042c6")]
pub mod system_stm32f_0xx;
#[cfg(feature = "stm32f042c6")]
pub use system_stm32f_0xx as mcu_system;

/* ====== STM32F100ZE ====== */

#[cfg(feature = "stm32f100ze")]
pub mod system_stm32f_100;
#[cfg(feature = "stm32f100ze")]
pub use system_stm32f_100 as mcu_system;

/* ====== STM32F103RC ====== */

#[cfg(feature = "stm32f103rc")]
pub mod system_stm32f10_123;
#[cfg(feature = "stm32f103rc")]
pub use system_stm32f10_123 as mcu_system;

/* ====== STM32F107VC ====== */

#[cfg(feature = "stm32f107vc")]
pub mod system_stm32f10_57;
#[cfg(feature = "stm32f107vc")]
pub use system_stm32f10_57 as mcu_system;

/* ====== STM32F217ZG ====== */

#[cfg(feature = "stm32f217zg")]
pub mod system_stm32f_2xx;
#[cfg(feature = "stm32f217zg")]
pub use system_stm32f_2xx as mcu_system;

/* ====== STM32F373VC ====== */

#[cfg(feature = "stm32f373vc")]
pub mod system_stm32f_3xx;
#[cfg(feature = "stm32f373vc")]
pub use system_stm32f_3xx as mcu_system;

/* ====== STM32F411RE ====== */

#[cfg(feature = "stm32f411re")]
pub mod system_stm32f_4xx;
#[cfg(feature = "stm32f411re")]
pub use system_stm32f_4xx as mcu_system;

/* ====== STM32F469II ====== */

#[cfg(feature = "stm32f469ii")]
pub mod system_stm32f_4hs;
#[cfg(feature = "stm32f469ii")]
pub use system_stm32f_4hs as mcu_system;

/* ====== STM32F723ZE ====== */

#[cfg(feature = "stm32f723ze")]
pub mod system_stm32f_7xx;
#[cfg(feature = "stm32f723ze")]
pub use system_stm32f_7xx as mcu_system;

/* ====== STM32G070RB ====== */

#[cfg(feature = "stm32g070rb")]
pub mod system_stm32g_0xx;
#[cfg(feature = "stm32g070rb")]
pub use system_stm32g_0xx as mcu_system;

/* ====== STM32H753ZI ====== */

#[cfg(feature = "stm32h753zi")]
pub mod system_stm32h_7xx;
#[cfg(feature = "stm32h753zi")]
pub use system_stm32h_7xx as mcu_system;

/* ====== STM32L073RZ ====== */

#[cfg(feature = "stm32l073rz")]
pub mod system_stm32l_0xx;
#[cfg(feature = "stm32l073rz")]
pub use system_stm32l_0xx as mcu_system;

/* ====== STM32L152RE ====== */

#[cfg(feature = "stm32l152re")]
pub mod system_stm32l_1xx;
#[cfg(feature = "stm32l152re")]
pub use system_stm32l_1xx as mcu_system;

/* ====== STM32L432KC ====== */

#[cfg(feature = "stm32l432kc")]
pub mod system_stm32l_4xx;
#[cfg(feature = "stm32l432kc")]
pub use system_stm32l_4xx as mcu_system;