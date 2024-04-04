#![no_std]
#![no_main]

use cortex_m_rt::entry;
pub use core::f64;


#[allow(unused_imports)]
use panic_halt; 

/* CONDITIONAL COMPILATION */

#[cfg(feature = "stm32f042c6")]
pub mod test_stm32f042c6;
#[cfg(feature = "stm32f042c6")]
pub use test_stm32f042c6 as mcu_test;
#[cfg(feature = "stm32f042c6")]
use stm32f0xx_hal as _;

#[cfg(feature = "stm32f100ze")]
pub mod test_stm32f100ze;
#[cfg(feature = "stm32f100ze")]
pub use test_stm32f100ze as mcu_test;
#[cfg(feature = "stm32f100ze")]
use stm32f1xx_hal as _;

#[cfg(feature = "stm32f103rc")]
pub mod test_stm32f103rc;
#[cfg(feature = "stm32f103rc")]
pub use test_stm32f103rc as mcu_test;
#[cfg(feature = "stm32f103rc")]
use stm32f1xx_hal as _;

#[cfg(feature = "stm32f107vc")]
pub mod test_stm32f107vc;
#[cfg(feature = "stm32f107vc")]
pub use test_stm32f107vc as mcu_test;
#[cfg(feature = "stm32f107vc")]
use stm32f1xx_hal as _;

#[cfg(feature = "stm32f217zg")]
pub mod test_stm32f217zg;
#[cfg(feature = "stm32f217zg")]
pub use test_stm32f217zg as mcu_test;
#[cfg(feature = "stm32f217zg")]
use stm32f2xx_hal as _;

#[cfg(feature = "stm32f373vc")]
pub mod test_stm32f373vc;
#[cfg(feature = "stm32f373vc")]
pub use test_stm32f373vc as mcu_test;
#[cfg(feature = "stm32f373vc")]
use stm32f3xx_hal as _;

#[cfg(feature = "stm32f411re")]
pub mod test_stm32f411re;
#[cfg(feature = "stm32f411re")]
pub use test_stm32f411re as mcu_test;
#[cfg(feature = "stm32f411re")]
use stm32f4xx_hal as _;

#[cfg(feature = "stm32f469ii")]
pub mod test_stm32f469ii;
#[cfg(feature = "stm32f469ii")]
pub use test_stm32f469ii as mcu_test;
#[cfg(feature = "stm32f469ii")]
use stm32f4xx_hal as _;

#[cfg(feature = "stm32f723ze")]
pub mod test_stm32f723ze;
#[cfg(feature = "stm32f723ze")]
pub use test_stm32f723ze as mcu_test;
#[cfg(feature = "stm32f723ze")]
use stm32f7xx_hal as _;

#[cfg(feature = "stm32g070rb")]
pub mod test_stm32g070rb;
#[cfg(feature = "stm32g070rb")]
pub use test_stm32g070rb as mcu_test;
#[cfg(feature = "stm32g070rb")]
use stm32g0xx_hal as _;

#[cfg(feature = "stm32h753zi")]
pub mod test_stm32h753zi;
#[cfg(feature = "stm32h753zi")]
pub use test_stm32h753zi as mcu_test;
#[cfg(feature = "stm32h753zi")]
use stm32h7xx_hal as _;

#[cfg(feature = "stm32l073rz")]
pub mod test_stm32l073rz;
#[cfg(feature = "stm32l073rz")]
pub use test_stm32l073rz as mcu_test;
#[cfg(feature = "stm32l073rz")]
use stm32l0xx_hal as _;

#[cfg(feature = "stm32l152re")]
pub mod test_stm32l152re;
#[cfg(feature = "stm32l152re")]
pub use test_stm32l152re as mcu_test;
#[cfg(feature = "stm32l152re")]
use stm32l1xx_hal as _;

#[cfg(feature = "stm32l432kc")]
pub mod test_stm32l432kc;
#[cfg(feature = "stm32l432kc")]
pub use test_stm32l432kc as mcu_test;
#[cfg(feature = "stm32l432kc")]
use stm32l4xx_hal as _;

/* =========== Test Workbench =========== */

#[entry]
fn main() -> ! 
{      
    mcu_test::delay::delay_test();
    
    //mcu_test::clock::clock_test();

    //mcu_test::interrupt::interrupt_test();

    //mcu_test::gpio::gpio_test();
        
    /* ===== FPU Test ====== */
    
    // let mut result: f64 = 0.0;

    // for i in 1..=1000 {
    //     result += (i as f64) * f64::consts::PI / (i as f64);
    // }

    loop{};
}

/* ======================================= */
