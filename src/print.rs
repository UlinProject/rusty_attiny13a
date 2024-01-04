
use core::ptr::addr_of;

use avr_progmem::{wrapper::ProgMem, raw::read_byte};
use crate::{bv::bv, uart::serial_write_byte, i2c::addr::I2CAddr, eeprom::EepromAddr};

pub trait PrintData {
	fn print(self);
}

impl PrintData for () {
	#[inline]
	fn print(self) {}
}

impl PrintData for &'_ u8 {
	#[inline]
	fn print(self) {
		PrintData::print(*self as u8)
	}
}

impl PrintData for u8 {
	#[inline]
	fn print(self) {
		serial_write_byte(self as u8)
	}
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct HexData<T>(T);

impl<T> From<T> for HexData<T> {
	#[inline(always)]
	fn from(value: T) -> Self {
		Self(value)
	}
}

impl<T> HexData<T> {
	#[inline(always)]
	pub const fn new(a: T) -> Self {
		Self(a)
	}
}

impl PrintData for HexData<bool> {
	fn print(self) {
		HexData::new(self.0 as u8).print()
	}
}

impl PrintData for HexData<I2CAddr> {
	fn print(self) {
		HexData::new(self.0.rm_control_bit() as u8).print()
	}
}

impl PrintData for HexData<EepromAddr> {
	fn print(self) {
		HexData::new(self.0.read() as u8).print()
	}
}

impl PrintData for HexData<u8> {
	fn print(self) {
		#[link_section = ".progmem.hexdataprint"]
		static HDATA: [u8; 16] = *b"0123456789ABCDEF";
		
		fn print_hbyte(pos: u8) {
			let hptr = addr_of!(HDATA) as *const u8;
			
			unsafe {
				read_byte(hptr.add(pos as usize) as *const u8) as u8
			}.print();
		}
		
		b'0'.print();
		b'x'.print();
		print_hbyte(self.0 >> 4);
		print_hbyte(self.0 & 0xF);
	}
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BinData<T>(T);

impl<T> From<T> for BinData<T> {
	#[inline(always)]
	fn from(value: T) -> Self {
		Self(value)
	}
}

impl<T> BinData<T> {
	#[inline(always)]
	pub const fn new(a: T) -> Self {
		Self(a)
	}
}

impl PrintData for BinData<u8> {
	fn print(self) {
		let a: u8 = self.0;
		
		b'b'.print();
		let mut i = 0u8;
		let max = 8u8;
		while i < max {
			#[inline(always)]
			const fn make_bool(a: u8, i: u8, max: u8) -> bool {
				(a & bv(max - 1 - i)) != 0
			}
			
			let symb = if make_bool(a, i, max) {
				b'1'
			}else {
				b'0'
			};
			symb.print();
			
			i += 1;
		}
	}
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct NumData<T>(T);

impl<T> From<T> for NumData<T> {
	#[inline(always)]
	fn from(value: T) -> Self {
		Self(value)
	}
}

impl<T> NumData<T> {
	#[inline(always)]
	pub const fn new(a: T) -> Self {
		Self(a)
	}
}

impl PrintData for NumData<u8> {
	fn print(self) {
		let mut a: u8 = self.0;
		
		#[inline(always)]
		fn _print_onenum(a: u8) {
			(b'0' + a).print()
		}
		
		if a >= 100 {
			_print_onenum(a / 100);
			a %= 100;
			
			_print_onenum(a / 10);
			a %= 10;
			
			_print_onenum(a);
			return;
		}
		if a >= 10 {
			_print_onenum(a / 10);
			a %= 10;
		}
		_print_onenum(a);
	}
}

pub fn print_array<const N: usize>(array: [u8; N]) {
	let mut i = 0u8;
	let max = array.len() as u8;
	while max > i {
		let a = unsafe { *array.get_unchecked(i as usize) };
		i += 1;
		
		a.print();
	}
}

pub fn print_sarray(array: &[u8]) {
	let mut i = 0u8;
	let max = array.len() as u8;
	while max > i {
		let a = unsafe { *array.get_unchecked(i as usize) };
		i += 1;
		
		a.print();
	}
}

pub fn print_sarray2<const N: usize>(array: &[u8; N]) {
	let mut i = 0u8;
	let max = array.len() as u8;
	while max > i {
		let a = unsafe { *array.get_unchecked(i as usize) };
		i += 1;
		
		a.print();
	}
}

pub unsafe fn print_rawprogmem(
	mut a_pgm: *const u8,
	max_pgm: *const u8
) {
	while max_pgm > a_pgm {
		let a = read_byte(a_pgm);
		a_pgm = a_pgm.add(1);
		
		a.print();
	}
}

pub fn print_progmem<const N: usize>(array: ProgMem<[u8; N]>) {
	/*for a in array.iter() {
		a.print();
	}*/
	unsafe {
		let start_addr = array.as_ptr() as *const u8;
		let end_addr = start_addr.add(N);
		
		print_rawprogmem(
			start_addr,
			end_addr
		)
	}
}

impl PrintData for NumData<EepromAddr> {
	fn print(self) {
		NumData::new(self.0.read()).print()
	}
}

impl<const N: usize> PrintData for [u8; N] {
	fn print(self) {
		print_array(self)
	}
}

impl PrintData for &'_ [u8] {
	fn print(self) {
		print_sarray(self)
	}
}

impl PrintData for &'_ str {
	fn print(self) {
		print_sarray(self.as_bytes());
	}
}

impl<const N: usize> PrintData for &'_ [u8; N] {
	fn print(self) {
		print_sarray2(self)
	}
}

impl<const N: usize> PrintData for ProgMem<[u8; N]> {
	fn print(self) {
		print_progmem(self)
	}
}

#[macro_export]
macro_rules! print {
	[ @progmem: $($data:tt)* ] => {{
		const LEN: usize = ($($data)*).len();
		$crate::avr_progmem::progmem! {
			static progmem ADATA: [u8; LEN] = *($($data)*);
		}
		
		$crate::print!( ADATA );
		/*unsafe {
			let start_addr = ADATA.as_ptr() as *const u8;
			let end_addr = start_addr.add(LEN);
			
			$crate::print::print_rawprogmem(
				start_addr,
				end_addr
			)
		}*/
	}};
	[ @num: $($data:tt)* ] => {{
		let num = $crate::print::NumData::new( $($data)* );
		
		$crate::print!(num);
	}};
	
	[ @bin: $($data:tt)* ] => {{
		let bin = $crate::print::BinData::new( $($data)* );
		
		$crate::print!(bin);
	}};
	[ @hex: $($data:tt)* ] => {{
		let hex = $crate::print::HexData::new( $($data)* );
		
		$crate::print!(hex);
	}};
	[ $($symb:tt)* ] => {{
		$crate::print::PrintData::print($($symb)*);
	}};
	
	[] => [];
}

#[macro_export]
macro_rules! println {
	[ $($all:tt)+ ] => {{
		$crate::print! {
			$($all)*
		}
		$crate::print!(b'\n');
	}};
	[] => {
		$crate::print!(b'\n');
	};
}
