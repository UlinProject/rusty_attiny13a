
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_attiny13a::{uart::{serial_init, serial_read}, osccal::loadosccal_from_eeprom, int::NoIntZone, print};

// 552 byte flash!

// By a strange coincidence, it was not possible to read above 115200.
// below speeds that definitely work.

// UART_BAUD=115200 cargo run --release --example uart_rw
// UART_BAUD=57600 cargo run --release --example uart_rw
// UART_BAUD=9600 cargo run --release --example uart_rw
// UART_BAUD=4800 cargo run --release --example uart_rw
// UART_BAUD=CUSTOM cargo run --release --example uart_rw

// if you want stability at a certain speed you can adjust the calibration index.
//
// The parity bit for UART is implemented only for TX.

#[no_mangle]
pub extern "C" fn main() -> ! {
	let nointzone = NoIntZone::make();
	loadosccal_from_eeprom(&nointzone);
	serial_init();
	
	/*
		read the data packet (4 bytes) and output it.
	*/
	loop {
		// Due to a strange circumstance, 
		// it is impossible to read more than 18 elements, maximum 18.
		// (this is also limited in function)
		let array = serial_read::<4>(); // 4 -> [u8; 4], max: searial_read::<18>()
		
		print!(array);
		print!(b"\r\n");
	}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}