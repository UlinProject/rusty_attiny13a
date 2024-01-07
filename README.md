# rusty_attiny13a

A weekend experiment using the Rust programming language on small attiny13a microcontrollers.

<img src="img/ferris.png" width="220" height="160" alt="print_logo, print_u16counter, flip"></img>

<H3>attiny13a:</H3>

| name | value |
| ---- | ----- |
| cpu | <b>9.6Mhz (internal, +osccal(pre-recorded in eeprom))</b> |
| mem | <b>64bytes</b> |
| flash | <b>1kb</b> |
| eeprom | <b>64bytes</b> |
| pio | <b>PB0-TX, PB1-RX, PB4-SDA, PB3-SCL</b> |

<H2>UART:</H2>

1. Supports I/O at a specified speed using a parity bit (write-only bit).

3. Delays are implemented only in software (the timer is not used).

4. Supports recording to multiple I/O ports simultaneously.

5. Support for burst or single-byte reads.

examples: `uart`, `uart_rw`, `uart_rw_debug`, `uart_rw_onebyte`

env: 
| env_name    | def_env_value | possible_env_value | description |
| ----------- | ------------- | --------- | ----------- |
| UART_BAUD   | `115200`       | `460800`, `230400`, `115200`, `57600`, `9600`, `4800`, `<CUSTOM_BAUD>` | Sets UART BAUD to default. |
| UART_PARITY | `EVEN`         | Even: (`1`, `EVEN`, `even`), Odd: (`2`, `ODD`, `odd`), Skip: (`0`, `SKIP`, `skip`) | Specifies the requirement to set the parity bit when writing. |

Launch example:
```bash
UART_BAUD=115200 UART_PARITY=0 cargo run --release --example uart
```

<H2>I2C:</H2>

1. Support for the I2C bus with setting the desired frequency (it was not possible to set the exact frequency due to flash memory limitations, for example, at a frequency of 400 kHz we get +-355 kHz at the output). <i>(100khz/400khz/800khz/... also supported.)</i>

2. Support for the ACK bit, and with it the ability to determine the success of a transmission.

<b>i2c_scan:</b>

A classic example of i2c bus scanning.

```bash
UART_BAUD=115200 cargo run --release --example i2c_scan
```

<H2>SSD1306:</H2>

<b>print_logo, print_u16counter, flip:</b>

<img src="photo/ssd1306.jpg" width="350" height="200" alt="print_logo, print_u16counter, flip"></img>

```bash
cargo run --release --example ssd1306_x32_counter
```


### License

Copyright 2023(+3h = 2024) #UlinProject (Denis Kotlyarov) Денис Котляров

Licensed under the MIT License

Licensed under the Apache License, Version 2.0
