
use core::marker::ConstParamTy;

use avrd::current::{PORTB, PINB, DDRB};
use crate::{volatile, bv::{bv, invers_bv}};

/// I/O ports.
#[derive(ConstParamTy, PartialEq, Eq)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Pio {
	PB0 = 0,
	PB1 = 1,
	PB2 = 2,
	PB3 = 3,
	PB4 = 4,
}

impl Pio {
	#[inline(always)]
	pub fn output(self) {
		volatile! {
			*DDRB = volatile!(*DDRB) | self._bv();
		}
	}
	
	#[inline(always)]
	pub fn input(self) {
		volatile! {
			*DDRB = volatile!(*DDRB) & self._inverse_bv();
		}
	}
	
	#[inline(always)]
	pub fn input_pullup(self) {
		self.input();
		self.high();
	}
	
	#[inline(always)]
	pub fn high(self) {
		volatile! {
			*PORTB = volatile!(*PORTB) | self._bv();
		}
	}
	
	#[inline(always)]
	pub fn low(self) {
		volatile! {
			*PORTB = volatile!(*PORTB) & self._inverse_bv();
		}
	}
	
	#[inline(always)]
	pub fn invert(self) {
		volatile! {
			*PORTB = volatile!(*PORTB) ^ self._bv();
		}
	}
	
	#[inline(always)]
	pub fn dig_read(self) -> bool {
		(volatile!(*PINB) & self._bv()) != 0
	}
	
	#[inline(always)]
	pub const fn pin(self) -> u8 {
		self as u8
	}
	
	#[inline(always)]
	pub const fn _bv(self) -> u8 {
		bv(self.pin())
	}
	
	#[inline(always)]
	pub const fn _inverse_bv(self) -> u8 {
		invers_bv(self.pin())
	}
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PioOutLevelState {
	a: u8,
}

impl PioOutLevelState {
	#[inline(always)]
	pub const unsafe fn from(a: u8) -> Self {
		Self {
			a,
		}
	}
	
	#[inline(always)]
	pub fn current() -> Self {
		let pio = volatile!(*PORTB);
		
		unsafe {
			Self::from(pio)
		}
	}
	
	#[inline(always)]
	pub fn reload(&mut self) {
		self.a = volatile!(*PORTB);
	}
	
	#[inline(always)]
	pub const fn on(mut self, pin: Pio) -> Self {
		self.a |= pin._bv();
		
		self
	}
	
	#[inline(always)]
	pub const fn off(mut self, pin: Pio) -> Self {
		self.a &= pin._inverse_bv();
		
		self
	}
	
	
	#[inline(always)]
	pub const fn c_on<const PIN: Pio>(mut self) -> Self {
		self.a |= PIN._bv();
		
		self
	}
	
	#[inline(always)]
	pub const fn c_off<const PIN: Pio>(mut self) -> Self {
		self.a &= PIN._inverse_bv();
		
		self
	}
	
	#[inline(always)]
	pub fn upload(self) {
		volatile!(*PORTB = self.a);
	}
	
	#[inline(always)]
	pub fn upload_inlinealways(self) { // inline always!
		self.upload()
	}
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PioInLevelState {
	a: u8,
}

impl PioInLevelState {
	#[inline(always)]
	pub const unsafe fn from(a: u8) -> Self {
		Self {
			a,
		}
	}
	
	#[inline]
	pub const fn zeroed() -> Self {
		unsafe { Self::from(0) }
	}
	
	#[inline(always)]
	pub fn current() -> Self {
		let pio = volatile!(*PINB);
		
		unsafe {
			Self::from(pio)
		}
	}
	
	#[inline(always)]
	pub fn reload(&mut self) {
		self.a = volatile!(*PINB);
	}
	
	#[inline(always)]
	pub fn reload_inlinealways(&mut self) {
		self.reload()
	}
	
	#[inline]
	pub fn is_fn<R>(
		self,
		pin: Pio,
		on: impl FnOnce() -> R,
		off: impl FnOnce() -> R
	) -> R {
		if (self.a & pin._bv()) != 0 {
			on()
		}else {
			off()
		}
	}
	
	#[inline]
	pub fn c_is_fn<R, const PIN: Pio>(
		self,
		on: impl FnOnce() -> R,
		off: impl FnOnce() -> R
	) -> R {
		if (self.a & PIN._bv()) != 0 {
			on()
		}else {
			off()
		}
	}
	
	#[inline(always)]
	pub const fn is(
		self,
		pin: Pio
	) -> bool {
		if (self.a & pin._bv()) != 0 {
			true
		}else {
			false
		}
	}
	
	#[inline(always)]
	pub const fn c_is<const PIN: Pio>(self) -> bool {
		if (self.a & PIN._bv()) != 0 {
			true
		}else {
			false
		}
	}
	
	#[inline(always)]
	pub fn c_static_is<const PIN: Pio>() -> bool {
		let pio = volatile!(*PINB);
		
		if (pio & PIN._bv()) != 0 {
			true
		}else {
			false
		}
	}
}

