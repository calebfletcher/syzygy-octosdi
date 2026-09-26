# SmartVIO LED and VIO ADC firmware

RTIC 2 cycles the three MCU-driven LEDs, one at a time, with a three-second full cycle. Embassy STM32 owns the GPIO, ADC, and clock setup; SysTick provides RTIC's monotonic clock. defmt logs are sent over RTT through the ST-Link.

| Indicator | Part | STM32 pin | Time on each cycle |
| --- | --- | --- | --- |
| VIO_GOOD | D12 | PA2 | 1 s |
| USER_LED_0 | D11 | PB4 | 1 s |
| USER_LED_1 | D10 | PB5 | 1 s |

The indicators are active high. **PA2 is also the board's VIO_GOOD control net**, so this example toggles that net and any circuitry it enables. Other board LEDs are driven by power or signal circuits, not directly by the STM32.

VIO is wired directly to PA1 / ADC1_IN1. The firmware samples VIO and the internal voltage reference every 100 ms and logs VIO in millivolts. It uses the factory VREF calibration to account for the actual ADC supply voltage.

From this directory:

    cargo build --release
    cargo run --release

cargo run flashes the STM32G031G8Ux using probe-rs and displays RTT logs. The target is thumbv6m-none-eabi and the probe uses SWD.

