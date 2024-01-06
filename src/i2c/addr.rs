
use core::marker::ConstParamTy;

const MAX_I2C_ADDR: usize = 0b01111111;

/// 7bit addr + 1bit typewrite
#[derive(ConstParamTy)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct I2CAddr {
	addr: u8,
}

impl I2CAddr {
	#[inline]
	pub const fn new_addrwrite_or_abort(addr: u8) -> Self {
		{ // const_check
			const CHECK_BYTE: [(); MAX_I2C_ADDR +1] = [(); MAX_I2C_ADDR +1];
			
			let _a = CHECK_BYTE[addr as usize];
		}
		
		let mut sself = Self {
			addr
		};
		sself._insert_write();
		sself
	}
	
	#[inline]
	pub (crate) const fn _insert_write(&mut self) {
		self.addr <<= 1; // INSERT WRITE
	}
	
	#[inline]
	pub const fn start_addrwrite() -> I2CAddr {
		Self::new_addrwrite_or_abort(0)
	}
	
	#[inline]
	pub const fn end_addrwrite() -> I2CAddr {
		Self::new_addrwrite_or_abort(MAX_I2C_ADDR as _)
	}
	
	#[inline]
	pub const unsafe fn next(self) -> I2CAddr {
		let mut addr = self.addr();
		addr += 1;
		
		Self::new_addrwrite_or_abort(addr)
	}
	
	#[inline]
	pub const fn rm_control_bit(self) -> u8 {
		let mut addr = self.raw_read();
		addr >>= 1; // SKIP WRITE
		
		addr
	}
	
	#[inline]
	pub const fn addr(self) -> u8 {
		self.rm_control_bit()
	}
	
	#[inline(always)]
	pub const fn raw_read(self) -> u8 {
		self.addr
	}
	
	#[inline(always)]
	pub const fn raw_read2(&self) -> &u8 {
		&self.addr
	}
}
