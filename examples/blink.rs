
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_attiny13a::{osccal::loadosccal_from_eeprom, int::NoIntZone, delay::delay_noinline_100ms, pio::Pio};

#[no_mangle]
pub extern "C" fn main() -> ! {
	let nointzone = NoIntZone::make();
	loadosccal_from_eeprom(&nointzone);
	
	Pio::PB4.output();
	loop {
		Pio::PB4.invert();
		
		delay_noinline_100ms();
	}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}