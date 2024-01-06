
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_attiny13a::{osccal::loadosccal_from_eeprom, int::NoIntZone, pio::Pio, delay};

// 104bytes flash
// cargo run --release --example blink

#[no_mangle]
pub extern "C" fn main() -> ! {
	let nointzone = NoIntZone::make();
	loadosccal_from_eeprom(&nointzone);
	
	Pio::PB4.output();
	loop {
		Pio::PB4.invert();
		
		delay!(1000);
	}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}