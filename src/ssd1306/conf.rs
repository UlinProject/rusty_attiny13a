
use core::marker::ConstParamTy;

use crate::ssd1306::cmd::SSD1306Cmd;

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
				arr = &[
					//SSD1306Cmd::DisplayOff as _, see reset event
					SSD1306Cmd::SetDisplayClockDiv as _, 0x80,
					
					SSD1306Cmd::SetMultiplex as _, ((X32PAGES * 8) - 1),
						SSD1306Cmd::MemoryMode as _, 0x00,
						SSD1306Cmd::PageAddr as _, 0x00, (X32PAGES - 1),
						SSD1306Cmd::SetCOMpins as _, 0x02,
					SSD1306Cmd::ChargePump as _, 0x14, // int vcc
					
					SSD1306Cmd::SegRemap2 as _, 0xC8,
					//SSD1306Cmd::SetContrast as _, 0x7F, see reset event
					SSD1306Cmd::DisplayOn as _,
				][..];
				
			},
			false => {
				// only x64,
				
				const X64PAGES: u8 = SSD1306Config::X128_64.pages;
				arr = &[
					//SSD1306Cmd::DisplayOff as _, see reset event
					SSD1306Cmd::SetDisplayClockDiv as _, 0x80,
					
					SSD1306Cmd::SetMultiplex as _, ((X64PAGES * 8) - 1),
					SSD1306Cmd::ChargePump as _, 0x14, // int vcc
					
					SSD1306Cmd::SegRemap2 as _, 0xC8,
					// SSD1306Cmd::SetContrast as _, 0x7F, see reset event
					SSD1306Cmd::DisplayOn as _,
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
