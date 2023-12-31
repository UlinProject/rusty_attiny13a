
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_attiny13a::{int::NoIntZone, osccal::loadosccal_from_eeprom, i2c::I2CMaster, pio::Pio, ssd1306::{conf::SSD1306Config, SSD1306, Shift}, delay::delay_noinline_100ms};
use avr_progmem::progmem;

// 1014 byte flash (32bytes logo, + counter(+num fonts) + flip) + autodetect i2c_addr + i2c 400khz...

/*
	cargo run --release --example ssd1306_x32_counter
*/

// 128x32
pub const SSD1306_CONFIG: SSD1306Config = SSD1306Config::X128_32;

#[no_mangle]
pub extern "C" fn main() -> ! {
	let nointzone = NoIntZone::make();
	loadosccal_from_eeprom(&nointzone);
	
	let i2cmaster = {
		// PB4 - SDA
		// PB3 - SCL
		// i2c - 400khz
		// due to memory savings, it was not possible to stabilize the frequency 
		// (at 400 kHz it turns out to be approximately 355 kHz)
		//
		// Also available is 800 kHz and disabling frequency control using 0 kHz 
		// (then the i2c frequency will depend on the processor frequency).
		I2CMaster::<{Pio::PB4}, {Pio::PB3}, 0, 0>::init_400khz().gen()
	};
	let ssd1306 = loop { // wait display
		// autodetect addr, 0x32C or 0x32D
		// def_display type = x128x32;
		match SSD1306::<_, {SSD1306_CONFIG}>::search_addr(&i2cmaster) {
			None => continue,
			Some(a) => break a,
		}
	};
	
	ssd1306.clear();
	let cx = 5;
	let cy = 1;
	{ // print logo
		progmem! {
			static progmem RUST16: [u8; 32] = [
				// 'RUST-Language-Logo-Vector', 16x16px
				0x40, 0xe0, 0x3c, 0x3c, 0xf4, 0xf6, 0xf3, 0xb6, 0xb6, 0xf3, 0xf6, 0x64, 0x4c, 0x3c, 0xe0, 0x40,
				0x02, 0x07, 0x3c, 0x34, 0x27, 0x67, 0xc7, 0x45, 0x41, 0xc7, 0x67, 0x27, 0x34, 0x3f, 0x07, 0x02
			];
		}
		
		let mut logoiter = RUST16.iter();
		'printlogo: for y in 0..2 {
			for x in 0..16u8 {
				let alogo = match logoiter.next() {
					Some(a) => a,
					None => break 'printlogo,
				};
				ssd1306.set_xy(cx + x, cy + y);
				ssd1306.push(|w| {
					w.push_u8pixel(alogo);
				});
			}
		}
	}
	
	let mut counter: u16 = 0;
	let mut shift: u8 = 0;
	loop {
		ssd1306.set_xy(cx + 16 + 8, cy);
		ssd1306.push(|w| {
			w.draw_u16(counter);
			
			counter += 1;
		});
		ssd1306.shift(Shift::Vertical, shift);
		
		shift += 1;
		delay_noinline_100ms();
	}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}