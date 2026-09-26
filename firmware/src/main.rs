#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = embassy_stm32::pac, peripherals = false, dispatchers = [TIM16, TIM17])]
mod app {
    use embassy_stm32::{
        Peri,
        adc::{Adc, SampleTime, VREF_CALIB_MV},
        gpio::{Level, Output, Speed},
        peripherals::{ADC1, PA1},
    };
    use rtic_monotonics::stm32::prelude::*;

    stm32_tim2_monotonic!(Mono, 1_000_000);

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local) {
        let p = embassy_stm32::init(Default::default());
        Mono::start(embassy_stm32::rcc::clocks(&p.RCC).sys.to_hertz().unwrap().0);

        // Assume VIO is always good; keep both polarity outputs asserted.
        let vio_good = Output::new(p.PA2, Level::High, Speed::Low);
        let vio_good_n = Output::new(p.PA3, Level::Low, Speed::Low);
        let user_0 = Output::new(p.PB4, Level::Low, Speed::Low);
        let user_1 = Output::new(p.PB5, Level::Low, Speed::Low);
        blink::spawn(vio_good, vio_good_n, user_0, user_1).ok();
        log_vio::spawn(p.ADC1, p.PA1).ok();

        (Shared {}, Local {})
    }

    #[task]
    async fn blink(
        _cx: blink::Context,
        mut vio_good: Output<'static>,
        mut vio_good_n: Output<'static>,
        mut user_0: Output<'static>,
        mut user_1: Output<'static>,
    ) {
        loop {
            // Keep ownership of both outputs for the lifetime of the task.
            vio_good.set_high();
            vio_good_n.set_low();

            user_0.set_high();
            Mono::delay(1000.millis()).await;
            user_0.set_low();

            user_1.set_high();
            Mono::delay(1000.millis()).await;
            user_1.set_low();
        }
    }

    #[task]
    async fn log_vio(
        _cx: log_vio::Context,
        adc_peripheral: Peri<'static, ADC1>,
        mut vio: Peri<'static, PA1>,
    ) {
        let mut adc = Adc::new(adc_peripheral);
        let mut vref = adc.enable_vrefint();
        let vref_cal = vref.calibrated_value() as u32;

        loop {
            let vref_raw = adc.blocking_read(&mut vref, SampleTime::CYCLES160_5) as u32;
            let vio_raw = adc.blocking_read(&mut vio, SampleTime::CYCLES160_5) as u32;
            if vref_raw != 0 {
                let vdda_mv = VREF_CALIB_MV * vref_cal / vref_raw;
                let vio_mv = vio_raw * vdda_mv / 4095;
                defmt::info!("VIO: {} mV", vio_mv);
            } else {
                defmt::warn!("VIO ADC: VREF reading was zero");
            }
            Mono::delay(100.millis()).await;
        }
    }
}
