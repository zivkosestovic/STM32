#![no_std]
#![no_main]

use defmt_rtt as _; // global logger
use panic_probe as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0])]
mod app {
    use dwt_systick_monotonic::DwtSystick;

    use stm32f4xx_hal::{
        adc::{
            config::{AdcConfig, Dma, SampleTime, Scan, Sequence},
            Adc, Temperature,
        },
        dma::{config::DmaConfig, PeripheralToMemory, Stream0, StreamsTuple, Transfer},
        pac::{self, ADC1, DMA2},
        prelude::*,
        signature::{VtempCal110, VtempCal30},
    };

    const MONO_HZ: u32 = 84_000_000; // 8 MHz

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<MONO_HZ>;

    type DMATransfer =
        Transfer<Stream0<DMA2>, 0, Adc<ADC1>, PeripheralToMemory, &'static mut [u16; 2]>;

    #[shared]
    struct Shared {
        transfer: DMATransfer,
    }

    #[local]
    struct Local {
        buffer: Option<&'static mut [u16; 2]>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let device: pac::Peripherals = cx.device;

        let rcc = device.RCC.constrain();
        let _clocks = rcc
            .cfgr
            .use_hse(25.MHz())
            .require_pll48clk()
            .sysclk(MONO_HZ.Hz())
            .hclk(MONO_HZ.Hz())
            .pclk1(42.MHz())
            .pclk2(84.MHz())
            .freeze();

        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let systick = cx.core.SYST;

        let mono = DwtSystick::new(&mut dcb, dwt, systick, MONO_HZ);

        let gpiob = device.GPIOB.split();
        let voltage = gpiob.pb1.into_analog();

        let dma = StreamsTuple::new(device.DMA2);

        let config = DmaConfig::default()
            .transfer_complete_interrupt(true)
            .memory_increment(true)
            .double_buffer(false);

        let adc_config = AdcConfig::default()
            .dma(Dma::Continuous)
            .scan(Scan::Enabled);

        let mut adc = Adc::adc1(device.ADC1, true, adc_config);
        adc.configure_channel(&Temperature, Sequence::One, SampleTime::Cycles_480);
        adc.configure_channel(&voltage, Sequence::Two, SampleTime::Cycles_480);
        adc.enable_temperature_and_vref();

        // These buffers need to be 'static to use safely with the DMA - we can't allow them to be dropped while the DMA is accessing them.
        // The easiest way to satisfy that is to make them static, and the safest way to do that is with `cortex_m::singleton!`
        let first_buffer = cortex_m::singleton!(: [u16; 2] = [0; 2]).unwrap();
        let second_buffer = Some(cortex_m::singleton!(: [u16; 2] = [0; 2]).unwrap());
        // Give the first buffer to the DMA. The second buffer is held in an Option in `local.buffer` until the transfer is complete
        let transfer = Transfer::init_peripheral_to_memory(dma.0, adc, first_buffer, None, config);

        polling::spawn_after(1.secs()).ok();

        (
            Shared { transfer },
            Local {
                buffer: second_buffer,
            },
            init::Monotonics(mono),
        )
    }

    #[task(shared = [transfer])]
    fn polling(mut cx: polling::Context) {
        cx.shared.transfer.lock(|transfer| {
            transfer.start(|adc| {
                adc.start_conversion();
            });
        });

        polling::spawn_after(1.secs()).ok();
    }

    #[task(binds = DMA2_STREAM0, shared = [transfer], local = [buffer])]
    fn dma(cx: dma::Context) {
        let dma::Context { mut shared, local } = cx;
        let (buffer, sample_to_millivolts) = shared.transfer.lock(|transfer| {
            // When the DMA completes it will return the buffer we gave it last time - we now store that as `buffer`
            // We still have our other buffer waiting in `local.buffer`, so `take` that and give it to the `transfer`
            let (buffer, _) = transfer
                .next_transfer(local.buffer.take().unwrap())
                .unwrap();

            let sample_to_millivolts = transfer.peripheral().make_sample_to_millivolts();
            (buffer, sample_to_millivolts)
        });

        // Pull the ADC data out of the buffer that the DMA transfer gave us
        let raw_temp = buffer[0];
        let raw_volt = buffer[1];

        // Now that we're finished with this buffer, put it back in `local.buffer` so it's ready for the next transfer
        // If we don't do this before the next transfer, we'll get a panic
        *local.buffer = Some(buffer);

        let cal30 = VtempCal30::get().read() as f32;
        let cal110 = VtempCal110::get().read() as f32;

        let temperature = (110.0 - 30.0) * ((raw_temp as f32) - cal30) / (cal110 - cal30) + 30.0;
        let voltage = sample_to_millivolts(raw_volt);

        defmt::info!("temperature: {}, voltage: {}", temperature, voltage);
    }
}
