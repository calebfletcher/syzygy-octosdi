# SmartVIO LED and VIO ADC firmware

RTIC 2 keeps VIO_GOOD asserted and alternates the two user LEDs, one second each. Embassy STM32 owns the GPIO, ADC, and clock setup; TIM2 provides RTIC's monotonic clock. defmt logs are sent over RTT through the ST-Link.

| Signal or indicator | STM32 pin | Behaviour |
| --- | --- | --- |
| VIO_GOOD and its LED (D12) | PA2 | High continuously |
| VIO_GOOD_N | PA3 | Low continuously |
| USER_LED_0 (D11) | PB4 | On for 1 s, then off for 1 s |
| USER_LED_1 (D10) | PB5 | Off for 1 s, then on for 1 s |

VIO_GOOD and VIO_GOOD_N are asserted regardless of the measured voltage. VIO_GOOD also enables other board circuitry. Other board LEDs are driven by power or signal circuits, not directly by the STM32.

VIO is wired directly to PA1 / ADC1_IN1. The firmware samples VIO and the internal voltage reference every 100 ms and logs VIO in millivolts. It uses the factory VREF calibration to account for the actual ADC supply voltage.

From this directory:

    cargo build --release
    cargo run --release

cargo run flashes the STM32G031G8Ux using probe-rs and displays RTT logs. The target is thumbv6m-none-eabi and the probe uses SWD.
