
use core::ops::Deref;
use crate::i2c::{addr::I2CAddr, generic::I2CGenMaster};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SSD1306Addr {
	addr: I2CAddr,
}

impl Deref for SSD1306Addr {
	type Target = I2CAddr;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.read2()
	}
}

impl SSD1306Addr {
	pub const ADDR0X3_C: SSD1306Addr = Self::def_0x3c();
	pub const ADDR0X3_D: SSD1306Addr = Self::def_0x3d();
	
	#[inline]
	pub const unsafe fn from(a: I2CAddr) -> Self {
		Self {
			addr: a,
		}
	}
	
	#[inline]
	pub const fn def_0x3c() -> Self {
		const DEF: I2CAddr = I2CAddr::new_addrwrite_or_abort(0x3C);
		
		unsafe { Self::from(DEF) }
	}
	
	#[inline]
	pub const fn def_0x3d() -> Self {
		const DEF: I2CAddr = I2CAddr::new_addrwrite_or_abort(0x3D);
		
		unsafe { Self::from(DEF) }
	}
	
	#[inline]
	pub const fn read(self) -> I2CAddr {
		self.addr
	}
	
	#[inline]
	pub const fn read2(&self) -> &I2CAddr {
		&self.addr
	}
	
	#[inline(always)]
	pub fn is_exists(&self, i2c: impl I2CGenMaster) -> bool {
		i2c.is_exists(self.read())
	}
	
	#[inline]
	pub fn wait_init(&self, i2c: impl I2CGenMaster + Copy) {
		while !self.is_exists(i2c) {}
	}
	
	pub fn search(i2c: impl I2CGenMaster + Copy) -> Option<SSD1306Addr> {
		let mut addr = Self::def_0x3c();
		if addr.is_exists(i2c) {
			return Some(addr);
		}
		
		addr = Self::def_0x3d();
		if addr.is_exists(i2c) {
			return Some(addr);
		}
		
		None
	}
}