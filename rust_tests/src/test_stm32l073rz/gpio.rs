use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

pub fn gpio_test() {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure PA1 as output.
    let mut led = gpioa.pa4.into_push_pull_output();

    loop {
        // Set the LED high one million times in a row.
        for _ in 0..1_000_000 {
            led.set_high().unwrap();
        }

        // Set the LED low one million times in a row.
        for _ in 0..1_000_000 {
            led.set_low().unwrap();
        }
    }
}
