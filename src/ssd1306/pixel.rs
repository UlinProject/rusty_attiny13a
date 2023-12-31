
use core::marker::ConstParamTy;

/// (Please note that the SSD1306 pixels are not points, but a defined area.)
#[derive(ConstParamTy, PartialEq, Eq)]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct SSD1306Pixel {
	a: u8
}

impl From<u8> for SSD1306Pixel {
	#[inline(always)]
	fn from(a: u8) -> Self {
		Self::new(a)
	}
}

impl SSD1306Pixel {
	pub const FULL_WHITE: SSD1306Pixel = SSD1306Pixel::full_white();
	pub const FULL_BLACK: SSD1306Pixel = SSD1306Pixel::full_black();

	#[inline]
	pub const fn full_white() -> Self {
		Self::new(255)
	}
	
	#[inline]
	pub const fn full_black() -> Self {
		Self::new(0)
	}
	
	#[inline]
	pub const fn new(a: u8) -> Self {
		Self {
			a,
		}
	}
	
	#[inline(always)]
	pub const fn read(self) -> u8 {
		self.a
	}
	
	#[inline(always)]
	pub const fn read2(&self) -> &u8 {
		&self.a
	}
}