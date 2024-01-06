
use crate::ssd1306::cmd::SSD1306Cmd;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SSD1306ScrollSpeed {
	S0 = 0x00,
	S1 = 0x01,
	S2 = 0x02,
	S3 = 0x03,
	S4 = 0x04,
	S5 = 0x05,
	S6 = 0x06,
	S7 = 0x07,
}

impl SSD1306ScrollSpeed {
	#[inline]
	pub const fn min() -> Self {
		Self::S0
	}
	
	#[inline]
	pub const fn max() -> Self {
		Self::S7
	}
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SSD1306ScrollDirection {
	HLeft = SSD1306Cmd::LeftHorizontalScroll as _,
	HRight = SSD1306Cmd::RightHorizontalScroll as _,
}
