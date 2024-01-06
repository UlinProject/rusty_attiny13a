
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_attiny13a::{uart::serial_init, int::NoIntZone, eeprom::{EepromAddr, eeprom_write_changes}, println, print, osccal::loadosccal_from_eeprom};
use avr_progmem::progmem;

mod bindata {
	#[allow(dead_code)]
	pub static RUST16: [u8; 32] = [
		// 'RUST-Language-Logo-Vector', 16x16px
		0x40, 0xe0, 0x3c, 0x3c, 0xf4, 0xf6, 0xf3, 0xb6, 0xb6, 0xf3, 0xf6, 0x64, 0x4c, 0x3c, 0xe0, 0x40,
		0x02, 0x07, 0x3c, 0x34, 0x27, 0x67, 0xc7, 0x45, 0x41, 0xc7, 0x67, 0x27, 0x34, 0x3f, 0x07, 0x02
	];
}

// 796bytes
// !TODO, unfinished

#[no_mangle]
pub extern "C" fn main() -> ! {
	let nointzone = NoIntZone::make();
	loadosccal_from_eeprom(&nointzone);
	serial_init();
	
	// data
	progmem! {
		static progmem EEPROM_DATA: [u8; 32] = bindata::RUST16;
	}
	println!(progmem: b"\n\r!TODO, unfinished.\n"); // , I'm starting to record the eeprom
	{ // Write
		let mut eeprom_addr = EepromAddr::get_start_data();
		let mut progmem_iter = EEPROM_DATA.iter();
		'weeprom: loop {
			let aprogmemdata = match progmem_iter.next() {
				Some(a) => a,
				None => break 'weeprom, // There is no more data to record.
			};
			print!({progmem: b"\r#["} {num: eeprom_addr} {progmem: b"]: = "} {X: aprogmemdata});
			
			if eeprom_write_changes(
				&nointzone,
				eeprom_addr,
				aprogmemdata
			).wait() {
				print!(progmem: b" (UPDATED)");
			}
			println!(b';');
			
			match eeprom_addr.next() {
				Some(nadrr) => eeprom_addr = nadrr,
				None => break 'weeprom, // EEPROM memory is exhausted.
			}
		}
	}
	drop(nointzone);
	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}