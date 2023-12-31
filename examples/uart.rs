
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_attiny13a::{uart::serial_init, osccal::loadosccal_from_eeprom, int::NoIntZone, print, delay};

// 350bytes flash!
// 326bytes flash (UART_PARITY=SKIP!)

// UART_BAUD=460800 cargo run --release --example uart
// UART_BAUD=230400 cargo run --release --example uart
// UART_BAUD=115200 cargo run --release --example uart
// UART_BAUD=57600 cargo run --release --example uart
// UART_BAUD=9600 cargo run --release --example uart
// UART_BAUD=4800 cargo run --release --example uart
// UART_BAUD=CUSTOM cargo run --release --example uart

// 921600 also works but is unstable, 
//
// if you want stability at a certain speed you can adjust the calibration index.
//
// The parity bit for UART is implemented only for TX.
// UART_BAUD=115200 UART_PARITY=SKIP cargo run --release --example uart
// (Disabling parity results in greater flash savings.)

#[no_mangle]
pub extern "C" fn main() -> ! {
	let nointzone = NoIntZone::make();
	loadosccal_from_eeprom(&nointzone);
	serial_init();
	/*
		TX PB0 // (code supports more than one simultaneous TX). (see serial_write_byte)
		RX PB1
		+ Even Parity
	*/
	loop {
		print!(progmem: b"\r\nHello world");
		
		delay!(1000);
	}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}