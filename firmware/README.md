# SmartVIO LED firmware

RTIC 2 cycles the three MCU-driven LEDs once per second, one at a time. Embassy STM32 owns the GPIO and clock setup; SysTick provides RTIC's monotonic clock. defmt logs are sent over RTT through the ST-Link.

| Indicator | Part | STM32 pin | Time on each cycle |
| --- | --- | --- | --- |
| VIO_GOOD | D12 | PA2 | 333 ms |
| USER_LED_0 | D11 | PB4 | 333 ms |
| USER_LED_1 | D10 | PB5 | 334 ms |

The indicators are active high. **PA2 is also the board's VIO_GOOD control net**, so this example toggles that net and any circuitry it enables. Other board LEDs are driven by power or signal circuits, not directly by the STM32.

From this directory:

    cargo build --release
    cargo run --release

cargo run flashes the STM32G031G8Ux using probe-rs and displays RTT logs. The target is thumbv6m-none-eabi and the probe uses SWD.
