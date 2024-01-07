
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_attiny13a::{uart::{serial_init, serial_read}, osccal::loadosccal_from_eeprom, int::NoIntZone, print, crc::crc8_array};

// 826 bytes flash!

// By a strange coincidence, it was not possible to read above 115200.
// below speeds that definitely work.

// UART_BAUD=115200 cargo run --release --example uart_rw_debug
// UART_BAUD=57600 cargo run --release --example uart_rw_debug
// UART_BAUD=9600 cargo run --release --example uart_rw_debug
// UART_BAUD=4800 cargo run --release --example uart_rw_debug
// UART_BAUD=CUSTOM cargo run --release --example uart_rw_debug

// if you want stability at a certain speed you can adjust the calibration index.
//
// The parity bit for UART is implemented only for TX.
// 
// UART_BAUD=115200 UART_PARITY=SKIP cargo run --release --example uart_rw_debug
// (Disabling parity results in greater flash savings.)

#[no_mangle]
pub extern "C" fn main() -> ! {
	let nointzone = NoIntZone::make();
	loadosccal_from_eeprom(&nointzone);
	serial_init();
	
	/*
		read the data packet (10 bytes) and output it.
		
		TX PB0 // (code supports more than one simultaneous TX). (see serial_write_byte)
		RX PB1
		+ Even Parity
	*/
	loop {
		let array = serial_read::<10>();
		
		match array {
			Some(ref array) => { // Interestingly, referenceless operations require much more flash memory.
				print!(
					{array} {b'\t'} {X: crc8_array(array)} {b'['}
				);
				for a in array {
					print!({X: *a} {b", "});
				}
				print!(progmem: b"]\r\n");
			},
			None => print!(progmem: b"uart_error, \r\n"),
		}
	}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}