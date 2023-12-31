
use core::marker::ConstParamTy;
use crate::ssd1306::{pixel::SSD1306Pixel, fonts::font::{FONT, START_SYMB, LEN_SYMB}};

#[repr(transparent)]
#[derive(Clone, Copy)]
#[derive(ConstParamTy, PartialEq, Eq)]
pub struct FontPixel {
	pixels: [SSD1306Pixel; (LEN_SYMB) as usize]
}

impl FontPixel {
	#[inline]
	const fn _new(
		pixels: [SSD1306Pixel; (LEN_SYMB) as usize]
	) -> Self {
		Self {
			pixels
		}
	}
	
	pub const fn cnew(ascii_symb: u8) -> Self {
		let pos = ((ascii_symb - START_SYMB) as usize) * LEN_SYMB as usize;
		
		Self::_new([
			SSD1306Pixel::new(FONT[pos+0]),
			SSD1306Pixel::new(FONT[pos+1]),
			SSD1306Pixel::new(FONT[pos+2]),
			SSD1306Pixel::new(FONT[pos+3]),
			SSD1306Pixel::new(FONT[pos+4]),
		])
	}
	
	pub fn read(
		self,
		mut next: impl FnMut(SSD1306Pixel)
	) {
		unsafe {
			let mut i = 0u8;
			let max = 5u8;
			
			let mut draw;
			while max > i {
				draw = *self.pixels.get_unchecked(i as usize);
				next(draw);
				
				i += 1;
			}
			
			next(SSD1306Pixel::FULL_BLACK);
			/*next(*self.pixels.get_unchecked(0));
			next(*self.pixels.get_unchecked(1));
			next(*self.pixels.get_unchecked(2));
			next(*self.pixels.get_unchecked(3));
			next(*self.pixels.get_unchecked(4));*/
			//next(SSD1306Pixel::FULL_BLACK);
		}
	}
	
	/*#[inline(always)]
	pub const fn read(self) -> u8 {
		self.pixel.read()
	}
	
	#[inline(always)]
	pub const fn read2(&self) -> &u8 {
		self.pixel.read2()
	}*/
}