#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = stm32g0::stm32g031, peripherals = false, dispatchers = [TIM16])]
mod app {
    use embassy_stm32::gpio::{Level, Output, Speed};
    use rtic_monotonics::systick::prelude::*;

    systick_monotonic!(Mono, 1_000);

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let p = embassy_stm32::init(Default::default());
        Mono::start(
            cx.core.SYST,
            embassy_stm32::rcc::clocks(&p.RCC).sys.to_hertz().unwrap().0,
        );

        // All three MCU-driven LEDs are active high. PA2 also drives VIO_GOOD.
        let vio_good = Output::new(p.PA2, Level::Low, Speed::Low);
        let user_0 = Output::new(p.PB4, Level::Low, Speed::Low);
        let user_1 = Output::new(p.PB5, Level::Low, Speed::Low);
        defmt::info!("Cycling VIO_GOOD, USER_LED_0, USER_LED_1 at 1 Hz");
        blink::spawn(vio_good, user_0, user_1).ok();

        (Shared {}, Local {})
    }

    #[task]
    async fn blink(
        _cx: blink::Context,
        mut vio_good: Output<'static>,
        mut user_0: Output<'static>,
        mut user_1: Output<'static>,
    ) {
        loop {
            // The three dwell times add to one second. Only one LED is on at a time.
            vio_good.set_high();
            Mono::delay(1000.millis()).await;
            vio_good.set_low();

            user_0.set_high();
            Mono::delay(1000.millis()).await;
            user_0.set_low();

            user_1.set_high();
            Mono::delay(1000.millis()).await;
            user_1.set_low();
        }
    }
}
