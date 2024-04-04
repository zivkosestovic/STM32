use core::marker::PhantomData;
use core::mem;

use crate::gpio::gpioa::{PA0, PA1, PA2, PA3, PA6, PA7};
use crate::gpio::gpiob::{PB0, PB1, PB6, PB7, PB8, PB9};
use crate::gpio::{AltMode, Floating, Input};
use crate::rcc::Rcc;
use crate::stm32::{TIM10, TIM11, TIM2, TIM3, TIM4, TIM5};
use crate::time::Hertz;
use cast::{u16, u32};
use hal;

pub struct C1;
pub struct C2;
pub struct C3;
pub struct C4;

pub trait Pins<TIM> {
    type Channels;
    fn setup(&self);
}

pub trait PwmExt: Sized {
    fn pwm<PINS, T>(self, _: PINS, frequency: T, rcc: &mut Rcc) -> PINS::Channels
    where
        PINS: Pins<Self>,
        T: Into<Hertz>;
}

pub struct Pwm<TIM, CHANNEL> {
    _channel: PhantomData<CHANNEL>,
    _tim: PhantomData<TIM>,
}

macro_rules! channels {
    ($TIMX:ident, $af:expr, $c1:ty) => {
        impl Pins<$TIMX> for $c1 {
            type Channels = Pwm<$TIMX, C1>;

            fn setup(&self) {
                self.set_alt_mode($af);
            }
        }

        impl hal::PwmPin for Pwm<$TIMX, C1> {
            type Duty = u16;

            fn disable(&mut self) {
                unsafe {
                    (*$TIMX::ptr()).ccer.modify(|_, w| w.cc1e().clear_bit());
                }
            }

            fn enable(&mut self) {
                unsafe {
                    let tim = &*$TIMX::ptr();
                    tim.ccmr1_output
                        .modify(|_, w| w.oc1pe().set_bit().oc1m().bits(6));
                    tim.ccer.modify(|_, w| w.cc1e().set_bit());
                }
            }

            fn get_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).ccr1.read().ccr1().bits() }
            }

            fn get_max_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).arr.read().arr().bits() }
            }

            fn set_duty(&mut self, duty: u16) {
                unsafe { (*$TIMX::ptr()).ccr1.write(|w| w.ccr1().bits(duty)) }
            }
        }
    };
    ($TIMX:ident, $af:expr, $c1:ty, $c2:ty, $c3:ty, $c4:ty) => {
        channels!($TIMX, $af, $c1);

        impl Pins<$TIMX> for $c2 {
            type Channels = Pwm<$TIMX, C2>;

            fn setup(&self) {
                self.set_alt_mode($af);
            }
        }

        impl Pins<$TIMX> for $c3 {
            type Channels = Pwm<$TIMX, C3>;

            fn setup(&self) {
                self.set_alt_mode($af);
            }
        }

        impl Pins<$TIMX> for $c4 {
            type Channels = Pwm<$TIMX, C4>;

            fn setup(&self) {
                self.set_alt_mode($af);
            }
        }

        impl Pins<$TIMX> for ($c1, $c2) {
            type Channels = (Pwm<$TIMX, C1>, Pwm<$TIMX, C2>);

            fn setup(&self) {
                self.0.set_alt_mode($af);
                self.1.set_alt_mode($af);
            }
        }

        impl Pins<$TIMX> for ($c1, $c2, $c3, $c4) {
            type Channels = (
                Pwm<$TIMX, C1>,
                Pwm<$TIMX, C2>,
                Pwm<$TIMX, C3>,
                Pwm<$TIMX, C4>,
            );

            fn setup(&self) {
                self.0.set_alt_mode($af);
                self.1.set_alt_mode($af);
                self.2.set_alt_mode($af);
                self.3.set_alt_mode($af);
            }
        }

        impl hal::PwmPin for Pwm<$TIMX, C2> {
            type Duty = u16;

            fn disable(&mut self) {
                unsafe {
                    (*$TIMX::ptr()).ccer.modify(|_, w| w.cc2e().clear_bit());
                }
            }

            fn enable(&mut self) {
                unsafe {
                    let tim = &*$TIMX::ptr();
                    tim.ccmr1_output
                        .modify(|_, w| w.oc2pe().set_bit().oc2m().bits(6));
                    tim.ccer.modify(|_, w| w.cc2e().set_bit());
                }
            }

            fn get_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).ccr2.read().ccr2().bits() }
            }

            fn get_max_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).arr.read().arr().bits() }
            }

            fn set_duty(&mut self, duty: u16) {
                unsafe { (*$TIMX::ptr()).ccr2.write(|w| w.ccr2().bits(duty)) }
            }
        }

        impl hal::PwmPin for Pwm<$TIMX, C3> {
            type Duty = u16;

            fn disable(&mut self) {
                unsafe {
                    (*$TIMX::ptr()).ccer.modify(|_, w| w.cc3e().clear_bit());
                }
            }

            fn enable(&mut self) {
                unsafe {
                    let tim = &*$TIMX::ptr();
                    tim.ccmr2_output
                        .modify(|_, w| w.oc3pe().set_bit().oc3m().bits(6));
                    tim.ccer.modify(|_, w| w.cc3e().set_bit());
                }
            }

            fn get_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).ccr3.read().ccr3().bits() }
            }

            fn get_max_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).arr.read().arr().bits() }
            }

            fn set_duty(&mut self, duty: u16) {
                unsafe { (*$TIMX::ptr()).ccr3.write(|w| w.ccr3().bits(duty)) }
            }
        }

        impl hal::PwmPin for Pwm<$TIMX, C4> {
            type Duty = u16;

            fn disable(&mut self) {
                unsafe {
                    (*$TIMX::ptr()).ccer.modify(|_, w| w.cc4e().clear_bit());
                }
            }

            fn enable(&mut self) {
                unsafe {
                    let tim = &*$TIMX::ptr();
                    tim.ccmr2_output
                        .modify(|_, w| w.oc4pe().set_bit().oc4m().bits(6));
                    tim.ccer.modify(|_, w| w.cc4e().set_bit());
                }
            }

            fn get_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).ccr4.read().ccr4().bits() }
            }

            fn get_max_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).arr.read().arr().bits() }
            }

            fn set_duty(&mut self, duty: u16) {
                unsafe { (*$TIMX::ptr()).ccr4.write(|w| w.ccr4().bits(duty)) }
            }
        }
    };
}

macro_rules! timers {
    ($($TIMX:ident: ($apb_clk:ident, $apbXenr:ident, $apbXrstr:ident, $timX:ident, $timXen:ident, $timXrst:ident),)+) => {
        $(
            impl PwmExt for $TIMX {
                fn pwm<PINS, T>(
                    self,
                    pins: PINS,
                    freq: T,
                    rcc: &mut Rcc,
                ) -> PINS::Channels
                where
                    PINS: Pins<Self>,
                    T: Into<Hertz>,
                {
                    $timX(self, pins, freq.into(), rcc)
                }
            }

            fn $timX<PINS>(
                tim: $TIMX,
                pins: PINS,
                freq: Hertz,
                rcc: &mut Rcc,
            ) -> PINS::Channels
            where
                PINS: Pins<$TIMX>,
            {
                pins.setup();
                rcc.rb.$apbXenr.modify(|_, w| w.$timXen().set_bit());
                rcc.rb.$apbXrstr.modify(|_, w| w.$timXrst().set_bit());
                rcc.rb.$apbXrstr.modify(|_, w| w.$timXrst().clear_bit());

                let clk = rcc.clocks.$apb_clk().0;
                let freq = freq.0;
                let ticks = clk / freq;
                let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                let arr = u16(ticks / u32(psc + 1)).unwrap();
                tim.psc.write(|w| unsafe { w.psc().bits(psc) });
                #[allow(unused_unsafe)]
                tim.arr.write(|w| unsafe { w.arr().bits(arr) });
                tim.cr1.write(|w| w.cen().set_bit());
                unsafe { mem::MaybeUninit::uninit().assume_init() }
            }
        )+
    }
}

channels!(
    TIM2,
    AltMode::TIM2,
    PA0<Input<Floating>>,
    PA1<Input<Floating>>,
    PA2<Input<Floating>>,
    PA3<Input<Floating>>
);
channels!(
    TIM3,
    AltMode::TIM3_5,
    PA6<Input<Floating>>,
    PA7<Input<Floating>>,
    PB0<Input<Floating>>,
    PB1<Input<Floating>>
);
channels!(
    TIM4,
    AltMode::TIM3_5,
    PB6<Input<Floating>>,
    PB7<Input<Floating>>,
    PB8<Input<Floating>>,
    PB9<Input<Floating>>
);
channels!(
    TIM5,
    AltMode::TIM3_5,
    PA0<Input<Floating>>,
    PA1<Input<Floating>>,
    PA2<Input<Floating>>,
    PA3<Input<Floating>>
);
channels!(TIM10, AltMode::TIM9_11, PA6<Input<Floating>>);
channels!(TIM11, AltMode::TIM9_11, PA7<Input<Floating>>);

timers! {
    TIM2: (apb1_clk, apb1enr, apb1rstr, tim2, tim2en, tim2rst),
    TIM3: (apb1_clk, apb1enr, apb1rstr, tim3, tim3en, tim3rst),
    TIM4: (apb1_clk, apb1enr, apb1rstr, tim4, tim4en, tim4rst),
    TIM5: (apb1_clk, apb1enr, apb1rstr, tim5, tim5en, tim5rst),
    TIM10: (apb2_clk, apb2enr, apb2rstr, tim10, tim10en, tm10rst),
    TIM11: (apb2_clk, apb2enr, apb2rstr, tim11, tim11en, tm11rst),
}
