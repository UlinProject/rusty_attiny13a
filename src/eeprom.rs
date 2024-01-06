
// not finalized.

use core::ops::Deref;
use avrd::current::{EECR, EERE, EEAR, EEDR, EEMWE, EEWE};

use crate::{int::NoIntZone, volatile};

pub const EEPROM_SIZE: u8 = 64;
pub const DEF_VALUE: u8 = 0xFF;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct EepromAddr {
	addr: u8
}

impl From<u8> for EepromAddr {
	#[inline(always)]
	fn from(value: u8) -> Self {
		Self::new_or_abort(value)
	}
}

impl EepromAddr {
	pub const OSCCAL_ADDR: EepromAddr = EepromAddr::new_or_abort(0);
	pub const START_ADDR: EepromAddr = EepromAddr::new_or_abort(1);
	pub const END_ADDR: EepromAddr = EepromAddr::new_or_abort(EEPROM_SIZE);
	
	#[inline(always)]
	pub const unsafe fn from_unchecked(addr: u8) -> Self {
		Self {
			addr
		}
	}
	
	/// The pointer is strictly greater than 0; 
	/// OSCCAL is usually located at address 0.
	#[inline(always)]
	pub const fn get_start_data() -> EepromAddr {
		Self::START_ADDR
	}
	
	#[inline]
	pub const fn new_or_abort(addr: u8) -> Self {
		{ // const_check
			const _CONST_CHECK: [(); (EEPROM_SIZE+1) as usize] = [(); (EEPROM_SIZE+1) as usize];
			// If you're reading here:
			// evaluation of constant value failed
			// index out of bounds: the length is 64 but the index is 255
			//
			// then know that your address is greater than the 
			// possible EEPROM address.
			let _a: () = _CONST_CHECK[addr as usize];
		}
		
		unsafe {
			Self::from_unchecked(addr)
		}
	}
	
	#[inline]
	pub fn next(mut self) -> Option<Self> {
		self.addr += 1;
		
		match self.addr > EEPROM_SIZE {
			true => None,
			false => Some(self)
		}
	}
	
	pub fn new<R>(
		addr: u8,
		next: impl FnOnce(Self) -> R,
		err: impl FnOnce() -> R
	) -> R {
		if addr > EEPROM_SIZE as _ {
			err()
		}else {
			let sself = unsafe {
				Self::from_unchecked(addr)
			};
			
			next(sself)
		}
	}
	
	#[inline(always)]
	pub const fn read(&self) -> u8 {
		self.addr
	}
}

impl Deref for EepromAddr {
	type Target = u8;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.addr
	}
}

pub fn eeprom_read(
	_int: &NoIntZone,
	
	uc_address: EepromAddr
) -> u8 {
	let uc_address: u8 = uc_address.read();
	let end_addr = EepromAddr::END_ADDR.read();
	
	volatile! {
		*EEAR = uc_address;
		*EECR = volatile!(_bv(*EERE));
		
		*EEAR = end_addr;
		
		*EEDR
	}
}

#[repr(transparent)]
#[must_use = "Don't forget, you need to wait for the recording to finish"]
pub struct WaitEndEepromWrite;

impl WaitEndEepromWrite {
	#[inline(always)]
	pub const fn new() -> Self {
		Self {}
	}
	
	pub fn wait(self) {
		eeprom_wwait();
	}
	
	#[inline(always)]
	pub fn skip(self) {}
}

fn eeprom_wwait() {
	while (volatile!(*EECR) & volatile!(_bv(*EEWE))) != 0 {}
}

pub fn eeprom_write(
	_int: &NoIntZone,
	
	uc_address: EepromAddr,
	value: u8
) -> WaitEndEepromWrite {
	let uc_address: u8 = uc_address.read();

	volatile! {
		*EEAR = uc_address; // upload addr
		*EEDR = value; // upload data
		
		*EECR |= volatile!(_bv(*EEMWE)); // enable write
		*EECR |= volatile!(_bv(*EEWE)); // start write
	}
	WaitEndEepromWrite
}

#[repr(transparent)]
#[must_use = "Don't forget, you need to wait for the recording to finish"]
pub struct AutoWaitEndEepromWriteChanges {
	opt: Option<WaitEndEepromWrite>,
}

impl AutoWaitEndEepromWriteChanges {
	#[inline(always)]
	const fn _new(opt: Option<WaitEndEepromWrite>) -> Self {
		Self {
			opt
		}
	}
	
	#[inline]
	pub const fn empty() -> Self {
		Self::_new(None)
	}
	
	#[inline]
	pub const fn new(a: WaitEndEepromWrite) -> Self {
		Self::_new(Some(a))
	}
	
	#[inline]
	pub const fn is_change(&self) -> bool {
		self.opt.is_some()
	}
	
	#[inline]
	pub fn wait(self) -> bool {
		match self.opt {
			Some(w) => {
				w.wait();
				
				true
			},
			None => false,
		}
	}
	
	#[inline(always)]
	pub fn skip(self) -> bool {
		match self.opt {
			Some(w) => {
				w.skip();
				
				true
			},
			None => false,
		}
	}
}

pub fn eeprom_write_changes(
	noint: &NoIntZone,
	
	uc_address: EepromAddr,
	value: u8
) -> AutoWaitEndEepromWriteChanges {
	let wait = match eeprom_read(noint, uc_address) != value {
		true => { // exp write
			Some(eeprom_write(noint, uc_address, value))
		}
		false => None,
	};
	
	AutoWaitEndEepromWriteChanges::_new(wait)
}

/*
examples:

for a in 1..64 {
	eeprom_write(a.into(), a+5).wait();
}

println!();
print!(b'\r');
for a in 1..64 {
	print!(b'\r');
	print!(@num: a);
	print!(b':');
	print!(b' ');
	
	print!(@hex: eeprom_read(a.into()));
	print!(b';');
	println!();
}
*/