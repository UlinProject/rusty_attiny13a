
use core::marker::ConstParamTy;

#[derive(ConstParamTy, PartialEq, Eq)]
#[derive(Clone, Copy)]
pub struct SSD1306Config {
	pages: u8,
	#[allow(unused)]
	multiple: u8,
	
	is_x32: bool,
	pub width: u8,
	pub height: u8,
}

impl SSD1306Config {
	pub const X128_32: SSD1306Config = SSD1306Config::new_128x32();
	pub const X128_64: SSD1306Config = SSD1306Config::new_128x64();
	
	#[inline]
	pub const fn new_128x32() -> Self {
		Self {
			pages: 4,
			multiple: 1,
			
			is_x32: true,
			width: 128,
			height: 32,
		}
	}
	
	#[inline]
	pub const fn new_128x64() -> Self {
		Self {
			pages: 4,
			multiple: 2,
			
			is_x32: false,
			width: 128,
			height: 64,
		}
	}
	
	pub fn read(
		self,
		mut write: impl FnMut(u8)
	) {
		/*
			An interesting effect: removing progmem can significantly save memory.
		*/
		let arr;
		match self.is_x32 {
			true => {
				// only x32
				
				const X32PAGES: u8 = SSD1306Config::X128_32.pages;
				/*
				const X32PAGES: u8 = SSD1306Config::X128_32.pages;
				progmem! {
					static progmem X32_SSD1306_INIT: [u8; 14] = [
						0xA8, ((X32PAGES * 8) - 1),
							0x20, 0x00,
							0x22, 0x00, (X32PAGES - 1),
							0xDA, 0x02,
						0x8D, 0x14,
						
						// AUTO_ON/OFF
						0xAF, // switch on OLED
						//0xAE, // OFF
						
						0xA1, 0xC8
					];
				}
				
				for a in X32_SSD1306_INIT.iter() {
					write(a);
				} */
				arr = &[
					0xA8, ((X32PAGES * 8) - 1),
						0x20, 0x00,
						0x22, 0x00, (X32PAGES - 1),
						0xDA, 0x02,
					0x8D, 0x14,
					
					// AUTO_ON/OFF
					0xAF, // switch on OLED
					//0xAE, // OFF
					
					0xA1, 0xC8
				][..];
				
			},
			false => {
				// only x64,
				
				const X64PAGES: u8 = SSD1306Config::X128_64.pages;
				/*progmem! {
					static progmem X64_SSD1306_INIT: [u8; 7] = [
						0xA8, ((X64PAGES * 8) - 1),
						0x8D, 0x14,
						
						// AUTO_ON/OFF
						0xAF,
						//0xAE, // OFF
						
						0xA1, 0xC8
					];
				}
				
				for a in X64_SSD1306_INIT.iter() {
					write(a);
				}*/
				
				arr = &[
					0xA8, ((X64PAGES * 8) - 1),
					0x8D, 0x14,
					
					// AUTO_ON/OFF
					0xAF,
					//0xAE, // OFF
					
					0xA1, 0xC8
				];
			}
		};
		
		let mut i = 0u8;
		let max = arr.len() as u8;
		
		while max > i {
			write(unsafe { *arr.get_unchecked(i as usize) });
			i += 1;
		}
	}
}
