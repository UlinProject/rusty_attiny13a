
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_attiny13a::{uart::serial_init, osccal::loadosccal_from_eeprom, int::NoIntZone, i2c::{I2CMaster, generic::I2CGenMaster}, pio::Pio, print, delay};

// 606bytes flash
// UART_BAUD=115200 cargo run --release --example i2c_scan
//
// UART_BAUD=115200 UART_PARITY=SKIP cargo run --release --example i2c_scan
// (Disabling parity results in greater flash savings.)

#[no_mangle]
pub extern "C" fn main() -> ! {
	let nointzone = NoIntZone::make();
	loadosccal_from_eeprom(&nointzone);
	
	/*
		TX PB0 // (code supports more than one simultaneous TX). (see serial_write_byte)
		RX PB1
		+ Even Parity
	*/
	serial_init();
	
	let i2cmaster = {
		// PB4 - SDA
		// PB3 - SCL
		// i2c - 100khz
		// due to memory savings, it was not possible to stabilize the frequency 
		// (at 400 kHz it turns out to be approximately 355 kHz)
		//
		// Also available is 800 kHz and disabling frequency control using 0 kHz 
		// (then the i2c frequency will depend on the processor frequency).
		I2CMaster::<{Pio::PB4}, {Pio::PB3}, 0, 0>::init_100khz().gen()
	};
	
	loop {
		print!(progmem: b"\n\rSearch:\n\r");
		
		i2cmaster.scan(
			|addr| { // exists,
				print!({b'#'} {X: addr} {progmem: b", exists!\n\r"});
			},
			|_addr| {} // no_exists
		);
		print!(progmem: b"End.\n\r");
		
		delay!(2000);
	}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}