use crate::pac::SYSCFG;
use crate::rcc::Enable;
use core::fmt;
use core::ops::Deref;

/// Extension trait that constrains the `SYSCFG` peripheral
pub trait SysCfgExt {
    /// Constrains the `SYSCFG` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> SysCfg;
}

impl SysCfgExt for SYSCFG {
    fn constrain(self) -> SysCfg {
        // Enable clock.
        unsafe {
            SYSCFG::enable_unchecked();
        }

        SysCfg(self)
    }
}

pub struct SysCfg(SYSCFG);

impl Deref for SysCfg {
    type Target = SYSCFG;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for SysCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SysCfg(SYSCFG)");
    }
}

impl fmt::Debug for SysCfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SysCfg").finish()
    }
}
