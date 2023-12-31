
use core::ptr::addr_of;

use avr_progmem::raw::read_value;

use crate::{i2c::generic::{I2CGenMaster, I2CGenTransaction}, ssd1306::{conf::SSD1306Config, addr::SSD1306Addr, pixel::SSD1306Pixel, fonts::pixel::FontPixel}, ssd1306_cfont};

pub mod addr;
pub mod conf;
pub mod pixel;
pub mod fonts {
	pub mod font;
	pub mod pixel;
}

/*
	Due to strict memory savings, the code had to be optimized in a unique way. :(
*/

pub struct SSD1306<I2C: I2CGenMaster, const CONFIG: SSD1306Config> {
	i2c: I2C,
	addr: SSD1306Addr,
}

const DC_BIT: u8 = 6;
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SSD1306TypeTransaction {
	Cmd = 0 << DC_BIT,
	Data = 1 << DC_BIT,
}

impl SSD1306TypeTransaction {
	#[inline(always)]
	pub const fn read(self) -> u8 {
		self as _
	}
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FullSlowFillingIsDouble {
	No = 1,
	On = 2,
}

impl Default for FullSlowFillingIsDouble {
	#[inline(always)]
	fn default() -> Self {
		Self::No
	}
}

impl<I2C: I2CGenMaster, const CONFIG: SSD1306Config> SSD1306<I2C, CONFIG> {
	#[inline]
	const fn _new(addr: SSD1306Addr, i2c: I2C) -> Self {
		Self {
			addr,
			i2c
		}
	}
	
	#[inline]
	pub const fn width(&self) -> u8 {
		CONFIG.width
	}
	
	#[inline]
	pub const fn height(&self) -> u8 {
		CONFIG.height
	}
	
	pub fn new(addr: SSD1306Addr, i2c: I2C) -> Option<Self> where I2C: Copy {
		// original code
		/*match addr.is_exists(i2c) {
			true => Some(Self::_new(addr, i2c)),
			false => None,
		}*/
		
		let sself = Self::_new(addr, i2c);
		match sself._upload_config() {
			true => Some(sself),
			false => None,
		}
	}
	
	/*pub fn new_fn(
		addr: SSD1306Addr, 
		i2c: I2C, 
		next: impl FnOnce(Self)) where I2C: Copy {
		match addr.is_exists(i2c) {
			true => next(Self::_new(addr, i2c)),
			false => {},
		}
	}*/
	
	#[inline]
	pub fn search_addr(i2c: I2C) -> Option<Self> {
		/* Using this practice saves memory */
		//
		let mut sself = Self::_new(SSD1306Addr::ADDR0X3_C, i2c);
		
		let mut is_etest_0x3d = true;
		'check_next_addr: loop {
			match sself._upload_config() {
				true => return Some(sself),
				false => {
					sself.addr = SSD1306Addr::ADDR0X3_D;
					
					if is_etest_0x3d {
						is_etest_0x3d = false;
						continue 'check_next_addr;
					}
					return None;
				}
			}
		}
		// original code
		/*match SSD1306Addr::search(&i2c) {
			Some(addr) => Some(Self::_new(addr, i2c)),
			None => None,
		}*/
	}
	
	/*fn _mode(
		&self, 
		mode: SSD1306TypeTransaction, 
		next: impl FnOnce(&'_ I2C::Transaction)
	) -> bool {
		let (mut result, transac) = unsafe { self.i2c.start(self.addr.read()) };
		if result {
			result = transac.write(mode.read());
			if result {
				next(&transac);
			}
		}
		unsafe { transac.stop(); }
		result
	}*/
	
	/*pub fn data(
		&self,
		next: impl FnOnce(SSD1306DataTransaction<I2C::Transaction>)
	) -> bool {
		self._mode(
			SSD1306TypeTransaction::Data, 
			|transac| next(SSD1306DataTransaction {
				i2c_transaction: transac
			})
		)
	}*/
	
	unsafe fn _cmd(&self) -> (bool, SSD1306CmdTransaction<I2C::Transaction>) {
		let (result, transac) = self._mode(SSD1306TypeTransaction::Cmd);
		
		(result, SSD1306CmdTransaction { i2c_transaction: transac })
	}
	
	unsafe fn _data(&self) -> (bool, SSD1306DataTransaction<I2C::Transaction>) {
		let (result, transac) = self._mode(SSD1306TypeTransaction::Data);
		
		(result, SSD1306DataTransaction { i2c_transaction: transac })
	}
	
	unsafe fn _mode(&self, ttype: SSD1306TypeTransaction) -> (bool, I2C::Transaction) {
		let (_, transac) = unsafe { self.i2c.start(self.addr.read()) };
		//if result {
			let result = transac.write(ttype.read());
		//}
		(result, transac)
	}
	
	fn _upload_config(&self) -> bool {
		let (mut result, w) = unsafe { self._cmd() };
		
		if result {
			CONFIG.read(
				|a| {
					result = unsafe { w.write(a) };
				}
			);
			
			//unsafe {
				//w.display_on();
				
				// ROTATE
				//w.unk_write(0x20);
				//w.unk_write(0x01); // SET MEM
				//result = w.unk_write(0x01); // ROTATE
			//}
		}
		w.stop();
		result
	}
	
	pub fn push_pixel(&self, data: SSD1306Pixel) -> bool {
		self.push(|w| {
			w.push_pixel(data);
		})
	}
	
	pub fn push(&self, w: impl FnOnce(&SSD1306DataTransaction<I2C::Transaction>)) -> bool {
		let (result, transac) = unsafe { self._data() };
		
		//if result {
			w(&transac);
		//}
		transac.stop();
		result
	}
	
	pub fn shift(&self, shift: Shift, pos: u8) {
		let (_, w) = unsafe { self._cmd() };
		
		//if result {
			let result = w.shift(shift, pos);
		//}
		w.stop();
		result
	}
	
	pub fn set_xy(&self, xpos: u8, ypos: u8) -> bool {
		let (_, w) = unsafe { self._cmd() };
		
		//if result {
			let result = w.set_xy(xpos, ypos);
		//}
		w.stop();
		result
	}
	
	pub fn clear(&self) {
		self.fullslow_filling(
			SSD1306Pixel::FULL_BLACK,
		);
	}
	
	pub fn fullslow_filling(&self, pixel: SSD1306Pixel) -> bool {
		// save memory :(
		//self.set_xy(0, 0);
		
		let (mut result, w) = unsafe { self._data() };
		
		//if result {
			{ // Clearing a display quickly is as easy as sending a lot of blocks.
				let mut i = 0u16;
				let max: u16 = (self.width() as u16) * (self.height() as u16);
				
				while max > i {
					result = w.push_pixel(pixel);
					i += 1;
				}
			}
		//}
		w.stop();
		result
	}
	
	pub fn push_xy_pixel(&self, x: u8, y: u8, pixel: SSD1306Pixel) {
		self.set_xy(x, y);
		self.push_pixel(pixel);
	}
	
	/*pub fn cmd(
		&self,
		next: impl FnOnce(SSD1306CmdTransaction<I2C::Transaction>)
	) -> bool {
		self._mode(
			SSD1306TypeTransaction::Cmd,
			|transac| next(SSD1306CmdTransaction {
				i2c_transaction: transac
			})
		)
	}
	
	pub fn upload_config(&self) -> bool {
		self.cmd(|w| {
			CONFIG.read(|a| unsafe { w.unk_write(a); });
			
			unsafe {
				//w.display_on();
				
				// ROTATE
				w.unk_write(0x20);
				w.unk_write(0x01); // SET MEM
				w.unk_write(0x01); // ROTATE
			}
		})
	}
	
	#[inline]
	pub fn set_xy(&self, xpos: u8, ypos: u8) -> bool {
		self.cmd(|a| a.set_xy(xpos, ypos))
	}
	
	#[inline]
	pub fn push_pixel(&self, data: SSD1306Pixel) -> bool {
		self.data(|a| { a.push_pixel(data); })
	}
	
	#[inline]
	pub fn push_whitepixel(&self) -> bool {
		self.push_pixel(SSD1306Pixel::FULL_WHITE)
	}
	
	#[inline]
	pub fn push_blackpixel(&self) -> bool {
		self.push_pixel(SSD1306Pixel::FULL_BLACK)
	}
	
	pub fn push_xy_pixel(&self, x: u8, y: u8, pixel: SSD1306Pixel) {
		self.set_xy(x, y);
		self.push_pixel(pixel);
	}
	
	pub fn clear(&self) {
		self.fullslow_filling(SSD1306Pixel::FULL_BLACK);
	}
	
	pub fn fullslow_filling(&self, pixel: SSD1306Pixel) {
		for xpos in 0..CONFIG.width {
			for ypos in 0..CONFIG.height {
				self.push_xy_pixel(xpos, ypos, pixel);
			}
		}
	}*/
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SSD1306DataTransaction<T: I2CGenTransaction> {
	i2c_transaction: T,
}

impl<T: I2CGenTransaction> SSD1306DataTransaction<T> {
	pub const WHITE: u8 = 255;
	pub const BLACK: u8 = 0;
	
	#[inline]
	pub fn stop(self) {
		unsafe { self.i2c_transaction.stop() }
	}
	
	#[inline]
	pub fn write(&self, a: u8) -> bool {
		self.i2c_transaction.write(a)
	}
	
	#[inline]
	pub fn push_pixel(&self, a: SSD1306Pixel) -> bool {
		self.push_u8pixel(a.read())
	}
	
	#[inline]
	pub fn push_u8pixel(&self, a: u8) -> bool {
		self.write(a)
	}
	
	#[inline]
	pub fn draw_font(&self, a: FontPixel) {
		a.read(|pixel| {
			self.push_pixel(pixel);
		})
	}
	
	pub fn draw_twopu16(&self, mut draw1: u16, draw2: u16) {
		// optimized for smaller size
		//
		let mut i = 0u8;
		let max = 2u8;
		while max > i {
			self.draw_u16(draw1);
			
			if i == 0 {
				self.draw_point();
			}
			draw1 = draw2;
			i += 1;
		}
	}
	
	pub fn draw_u16(&self, inum: u16) {
		// TODO
		let mut rnum = 0;
		let mut c = 0;
		{ // invers
			let mut num = inum;
			while num > 0 {
				rnum = rnum * 10 + (num % 10);
				num /= 10;
				
				c += 1;
			}
		}
		
		while c > 0 {
			self.draw_onesymbnum((rnum % 10) as _);
			rnum /= 10;
			
			c -= 1;
		}
	}
	
	/// N: 0..=9
	pub fn draw_onesymbnum(&self, n: u8) {
		// A small part of the fonts are stored in progmem.
		#[link_section = ".progmem.num"]
		static NUM_ARRAY: [FontPixel; 10] = [
			ssd1306_cfont!(b'0'),
			ssd1306_cfont!(b'1'),
			ssd1306_cfont!(b'2'),
			ssd1306_cfont!(b'3'),
			ssd1306_cfont!(b'4'),
			ssd1306_cfont!(b'5'),
			ssd1306_cfont!(b'6'),
			ssd1306_cfont!(b'7'),
			ssd1306_cfont!(b'8'),
			ssd1306_cfont!(b'9'),
		];
		
		self.draw_font(unsafe {
			read_value(
				(addr_of!(NUM_ARRAY) as *const FontPixel).add(
					n as usize
				)
			) 
		});
	}
	
	pub fn draw_point(&self) {
		// Yes, fonts can be made at runtime, but it requires a lot of resources :(
		self.draw_font(ssd1306_cfont!(b'.'));
	}
	
	#[inline]
	pub fn push_white(&self) -> bool {
		self.push_pixel(SSD1306Pixel::FULL_WHITE)
	}
	
	#[inline]
	pub fn push_black(&self) -> bool {
		self.push_pixel(SSD1306Pixel::FULL_BLACK)
	}
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Shift {
	Vertical = 0xD3,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum DisplayOnOff {
	On = 0xAF,
	Off = 0xAE,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SSD1306CmdTransaction<T: I2CGenTransaction> {
	i2c_transaction: T,
}

impl<T: I2CGenTransaction> SSD1306CmdTransaction<T> {
	#[inline]
	pub fn stop(self) {
		unsafe { self.i2c_transaction.stop() }
	}
	
	#[inline]
	pub unsafe fn write(&self, a: u8) -> bool {
		self.i2c_transaction.write(a)
	}
	
	pub fn set_xy(&self, xpos: u8, ypos: u8) -> bool {
		unsafe {
			let result;
			self.write(0xB0 | (ypos & 0x07));
			self.write(xpos & 0x0F);
			result = self.write(0x10 | (xpos >> 4));
			
			result
		}
	}
	
	pub fn display_onoff(&self, onoff: DisplayOnOff) {
		unsafe {
			self.write(onoff as _);
		}
	}
	
	pub fn shift(&self, shift: Shift, pos: u8) {
		unsafe {
			self.write(shift as _);
			self.write(pos);
		}
	}
	
	pub fn set_brig(&self, brig: u8) {
		unsafe {
			self.write(0x81);
			self.write(brig);
		}
	}
}
