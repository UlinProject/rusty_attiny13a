# rusty_attiny13a

A weekend experiment using the Rust programming language on small attiny13a microcontrollers.


<H3>attiny13a:</H3>

| name | value |
| ---- | ----- |
| cpu | <b>9.6Mhz (internal, +osccal(pre-recorded in eeprom))</b> |
| mem | <b>64bytes</b> |
| flash | <b>1kb</b> |
| eeprom | <b>64bytes</b> |
| pio | <b>PB0-TX, PB1-RX, PB4-SDA, PB3-SCL</b> |

<H2><i>UART:</i></H2>

1. Supports I/O at a specified speed using a parity bit (write-only bit).

3. Delays are implemented only in software (the timer is not used).

4. Supports recording to multiple I/O ports simultaneously.

5. Batch reading support. (no more than 18 bytes per transfer, the reason for the instability is not clear)

<b>print_hello_world:</b>
```bash
UART_BAUD=460800 cargo run --release --example uart
UART_BAUD=230400 cargo run --release --example uart
UART_BAUD=115200 cargo run --release --example uart
UART_BAUD=57600 cargo run --release --example uart
UART_BAUD=9600 cargo run --release --example uart
UART_BAUD=4800 cargo run --release --example uart
UART_BAUD=CUSTOM cargo run --release --example uart
```
<b>read_package_and_print_package:</b>
```bash
UART_BAUD=115200 cargo run --release --example uart_rw
UART_BAUD=57600 cargo run --release --example uart_rw
UART_BAUD=9600 cargo run --release --example uart_rw
UART_BAUD=4800 cargo run --release --example uart_rw
UART_BAUD=CUSTOM cargo run --release --example uart_rw
```
<i>(values above 115200 baud were not used for reading (unstable).)</i>

<i>(time calibration can be specified manually in the file.)</i>

<i>(values like 921600 are possible for output, but they are unstable.)</i>

<H2><i>I2C:</i></H2>


1. Support for the I2C bus with setting the desired frequency (it was not possible to set the exact frequency due to flash memory limitations, for example, at a frequency of 400 kHz we get +-355 kHz at the output). <i>(100khz/400khz/800khz/... also supported.)</i>

2. Support for the ACK bit, and with it the ability to determine the success of a transmission.

<b>i2c_scan:</b>

A classic example of i2c bus scanning.

```bash
UART_BAUD=115200 cargo run --release --example i2c_scan
```

<H2><i>SSD1306:</i></H2>

<b>print_logo, print_u16counter, flip:</b>

<img src="photo/ssd1306.jpg" width="350" height="200" alt="print_logo, print_u16counter, flip"></img>

```bash
cargo run --release --example ssd1306_x32_counter
```


### License

Copyright 2023(+3h = 2024) #UlinProject (Denis Kotlyarov) Денис Котляров

Licensed under the MIT License

Licensed under the Apache License, Version 2.0
