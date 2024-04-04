use stm32h7xx_hal::{block, prelude::*, timer::Timer};
pub fn gpio_test() {

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOB peripheral
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);

    // Configure gpio B pin 0 as a push-pull output.
    let mut led = gpioa.pa4.into_push_pull_output();

    // Configure the timer to trigger an update every second
    let mut timer = Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    timer.start(1.Hz());

    loop {
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
        block!(timer.wait()).unwrap();
    }
}