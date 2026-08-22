# syzygy-octosdi

SYZYGY™ TXR4 Compatible SDI-3G Pod, with quad inputs and quad outputs.

Compatible with the SYZYGY Specification Version 1.1.1. The "SYZYGY™" mark is owned by Opal Kelly.

![PCB Render](pcb_render.png)

## [Interactive BOM](https://html-preview.github.io/?url=https://github.com/calebfletcher/syzygy-octosdi/blob/main/ibom.html)

## SYZYGY Pinout
Pinout for CN1 (QTH-020-01-F-D-DP-A):

| Pin number | Pin name      | Pin net      |
|------------|---------------|--------------|
| 01         | SCL\_01       | VIO\_I2C.SCL |
| 02         | +5V\_02       | +5V          |
| 03         | SDA\_03       | VIO\_I2C.SDA |
| 04         | R\_GA\_04     | R\_GA        |
| 05         | RX0P\_05      | RX0.P        |
| 06         | TX0P\_06      | TX0.P        |
| 07         | RX0N\_07      | RX0.N        |
| 08         | TX0N\_08      | TX0.N        |
| 09         | RX1P\_09      | RX1.P        |
| 10         | TX1P\_10      | TX1.P        |
| 11         | RX1N\_11      | RX1.N        |
| 12         | TX1N\_12      | TX1.N        |
| 13         | REFCLKP\_13   | REFCLK.P     |
| 14         | S0\_14        | I2C.SCL      |
| 15         | REFCLKN\_15   | REFCLK.N     |
| 16         | S1\_16        | I2C.SDA      |
| 17         | S2\_17        | ~{RESET}     |
| 18         | S3\_18        | ~{INT}       |
| 19         | S4\_19        | NC           |
| 20         | S5\_20        | NC           |
| 21         | S6\_21        | NC           |
| 22         | S7\_22        | NC           |
| 23         | S8\_23        | NC           |
| 24         | S9\_24        | NC           |
| 25         | RX3P\_25      | RX3.P        |
| 26         | TX3P\_26      | TX3.P        |
| 27         | RX3N\_27      | RX3.N        |
| 28         | TX3N\_28      | TX3.N        |
| 29         | RX2P\_29      | RX2.P        |
| 30         | TX2P\_30      | TX2.P        |
| 31         | RX2N\_31      | RX2.N        |
| 32         | TX2N\_32      | TX2.N        |
| 33         | P2C\_CLKP\_33 | NC           |
| 34         | C2P\_CLKP\_34 | NC           |
| 35         | P2C\_CLKN\_35 | NC           |
| 36         | C2P\_CLKN\_36 | NC           |
| 37         | RSVD\_37      | NC           |
| 38         | RSVD\_38      | NC           |
| 39         | VIO1\_39      | VIO          |
| 40         | +3.3V\_40     | +3V3         |
| G1         | GND\_G1       | GND          |
| G2         | GND\_G2       | GND          |
| G3         | GND\_G3       | GND          |
| G4         | GND\_G4       | GND          |

## SmartVIO STM32 Pinout
Pinout for U11 (STM32G031G8Ux):

| Pin number | Pin name       | Pin net               |
|------------|----------------|-----------------------|
| 1          | PC14\_1        | NC                    |
| 2          | PC15\_2        | NC                    |
| 3          | VDD\_3         | +3V3                  |
| 4          | VSS\_4         | GND                   |
| 5          | PF2\_5         | SmartVIO/NRST         |
| 6          | ADC1\_IN0\_6   | R\_GA                 |
| 7          | ADC1\_IN1\_7   | VIO                   |
| 8          | PA2\_8         | VIO\_GOOD             |
| 9          | PA3\_9         | ~{VIO\_GOOD}          |
| 10         | PA4\_10        | NC                    |
| 11         | PA5\_11        | NC                    |
| 12         | PA6\_12        | NC                    |
| 13         | PA7\_13        | NC                    |
| 14         | PB0\_14        | Net-(U11-PB0)         |
| 15         | PB1\_15        | Net-(U11-PB1)         |
| 16         | PA8\_16        | NC                    |
| 17         | PC6\_17        | NC                    |
| 18         | PA9/PA11\_18   | NC                    |
| 19         | PA10/PA12\_19  | NC                    |
| 20         | SYS\_SWDIO\_20 | SmartVIO/SWDIO        |
| 21         | SYS\_SWCLK\_21 | SmartVIO/SWCLK        |
| 22         | PA15\_22       | NC                    |
| 23         | PB3\_23        | NC                    |
| 24         | PB4\_24        | SmartVIO/USER\_LED\_0 |
| 25         | PB5\_25        | SmartVIO/USER\_LED\_1 |
| 26         | I2C1\_SCL\_26  | VIO\_I2C.SCL          |
| 27         | I2C1\_SDA\_27  | VIO\_I2C.SDA          |
| 28         | PB8\_28        | NC                    |
