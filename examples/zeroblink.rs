
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_attiny13a::{pio::Pio, delay};

// 70bytes
// cargo run --release --example zeroblink

#[no_mangle]
pub extern "C" fn main() -> ! {
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