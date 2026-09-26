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
        i2c::{I2c, Master},
        mode::Blocking,
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
        let vio_good = Output::new(p.PA2, Level::Low, Speed::Low);
        let vio_good_n = Output::new(p.PA3, Level::High, Speed::Low);
        let user_0 = Output::new(p.PB4, Level::Low, Speed::Low);
        let user_1 = Output::new(p.PB5, Level::Low, Speed::Low);

        // VIO_I2C is externally bridged to APP_I2C for this bring-up test.
        let mut i2c = I2c::new_blocking(p.I2C1, p.PB6, p.PB7, Default::default());
        if let Some(ports) = read_expander(&mut i2c, 0x20) {
            for channel in 0..4 {
                let group = ports[channel / 2] >> ((channel % 2) * 4);
                defmt::info!(
                    "SDI In {}: BYPASS={} MUTE={} CD_N={}",
                    channel + 1,
                    group & 1 != 0,
                    group & 2 != 0,
                    group & 4 != 0,
                );
            }
        }
        if let Some(ports) = read_expander(&mut i2c, 0x21) {
            for channel in 0..4 {
                let group = ports[channel / 2] >> ((channel % 2) * 4);
                defmt::info!(
                    "SDI Out {}: SD_HD_N={} DISABLE_N={} EQ_EN_N={} OSP_N={}",
                    channel + 1,
                    group & 1 != 0,
                    group & 2 != 0,
                    group & 4 != 0,
                    group & 8 != 0,
                );
            }
        }

        blink::spawn(vio_good, vio_good_n, user_1).ok();
        monitor::spawn(p.ADC1, p.PA1, i2c, user_0).ok();

        (Shared {}, Local {})
    }

    fn read_expander(i2c: &mut I2c<'_, Blocking, Master>, address: u8) -> Option<[u8; 2]> {
        // PCAL6416A configuration bits: 1 = input. Set both eight-bit ports.
        for register in [0x06, 0x07] {
            if let Err(error) = i2c.blocking_write(address, &[register, 0xff]) {
                defmt::error!(
                    "I2C expander 0x{:02x}: input configuration failed: {:?}",
                    address,
                    error
                );
                return None;
            }
        }

        let mut configuration = [0; 2];
        for (index, register) in [0x06, 0x07].into_iter().enumerate() {
            if let Err(error) =
                i2c.blocking_write_read(address, &[register], &mut configuration[index..index + 1])
            {
                defmt::error!(
                    "I2C expander 0x{:02x}: configuration read failed: {:?}",
                    address,
                    error
                );
                return None;
            }
        }
        if configuration != [0xff, 0xff] {
            defmt::error!(
                "I2C expander 0x{:02x}: input configuration mismatch P0={} P1={}",
                address,
                configuration[0],
                configuration[1]
            );
            return None;
        }

        let mut inputs = [0; 2];
        for (index, register) in [0x00, 0x01].into_iter().enumerate() {
            if let Err(error) =
                i2c.blocking_write_read(address, &[register], &mut inputs[index..index + 1])
            {
                defmt::error!(
                    "I2C expander 0x{:02x}: input read failed: {:?}",
                    address,
                    error
                );
                return None;
            }
        }
        defmt::info!(
            "I2C expander 0x{:02x}: all inputs, raw P0={} P1={}",
            address,
            inputs[0],
            inputs[1]
        );
        Some(inputs)
    }

    #[task]
    async fn blink(
        _cx: blink::Context,
        mut vio_good: Output<'static>,
        mut vio_good_n: Output<'static>,
        mut user_1: Output<'static>,
    ) {
        loop {
            // Keep ownership of both outputs for the lifetime of the task.
            vio_good.set_high();
            vio_good_n.set_low();

            user_1.set_high();
            Mono::delay(1000.millis()).await;
            user_1.set_low();
        }
    }

    #[task]
    async fn monitor(
        _cx: monitor::Context,
        adc_peripheral: Peri<'static, ADC1>,
        mut vio: Peri<'static, PA1>,
        mut i2c: I2c<'static, Blocking, Master>,
        mut user_0: Output<'static>,
    ) {
        let mut adc = Adc::new(adc_peripheral);
        let mut vref = adc.enable_vrefint();
        let vref_cal = vref.calibrated_value() as u32;

        loop {
            // U6 port 0 bit 0 is SDI Out 1 SD/~HD; high lights USER_LED_0.
            let mut port_0 = [0u8; 1];
            match i2c.blocking_write_read(0x21, &[0x00], &mut port_0) {
                Ok(()) => {
                    let sd_hd_n = port_0[0] & 1 != 0;
                    if sd_hd_n {
                        user_0.set_high();
                    } else {
                        user_0.set_low();
                    }
                    defmt::info!("SDI Out 1 SD_HD_N={}", sd_hd_n);
                }
                Err(error) => {
                    user_0.set_low();
                    defmt::warn!("SDI Out 1 read failed: {:?}", error);
                }
            }

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
