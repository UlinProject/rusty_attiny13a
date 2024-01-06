
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
		// Using arrays, loops, etc. requires a lot more flash memory.
		/*#[link_section = ".progmem.hexdataprint"]
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
		print_hbyte(self.0 & 0xF);*/
		
		const fn make_hbyte(pos: u8) -> u8 {
			let mut a = b'0' + pos;
			
			if a > b'9' {
				a += 7;
			}
			
			a
		}
		
		b'0'.print();
		b'x'.print();
		make_hbyte(self.0 >> 4).print();
		make_hbyte(self.0 & 0xF).print();
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

impl PrintData for BinData<I2CAddr> {
	#[inline]
	fn print(self) {
		PrintData::print(
			BinData::new(self.0.addr())
		)
	}
}

impl PrintData for BinData<u8> {
	fn print(self) {
		let a: u8 = self.0;
		
		b'b'.print();
		let mut i = 0u8;
		let max = 8u8;
		while max > i {
			(b'0' + (((a & bv(max - i)) != 0) as u8)).print();
			
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
		// orig code
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
		/*
		#[inline(always)]
		fn _print_onenum(a: u8) {
			(b'0' + a).print()
		}
		let mut a: u8 = self.0;
		
		{
			'one_place_inend: {
				'two_place_inend: {
					if a >= 100 {
						_print_onenum(a / 100);
						a %= 100;
						
						break 'two_place_inend;
					}
					
					if a >= 10 {
						_print_onenum(a / 10);
						a %= 10;
						
						break 'one_place_inend;
					}
					
					break 'one_place_inend;
				}
				
				_print_onenum(a / 10);
				a %= 10;
				
				break 'one_place_inend;
			}
			_print_onenum(a);
		} */
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

impl PrintData for NumData<I2CAddr> {
	fn print(self) {
		NumData::new(self.0.rm_control_bit()).print()
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
	[ { $($block:tt)* } $($unk:tt)* ] => {
		$crate::print! {
			$($block)*
		}
		
		$crate::print! {
			$($unk)*
		}
	};
	
	[ progmem: $data: expr ] => {{
		const LEN: usize = $data.len();
		$crate::avr_progmem::progmem! {
			static progmem ADATA: [u8; LEN] = *$data;
		}
		
		$crate::print!( ADATA );
	}};
	[ num: $data: expr ] => {{
		let num = $crate::print::NumData::new( $data );
		
		$crate::print!(num);
	}};
	
	[ bin: $data: expr $( , $($unk:tt)* )? ] => {{
		let bin = $crate::print::BinData::new( $data );
		
		$crate::print!(bin);
	}};
	
	[ X: $data: expr $( , $($unk:tt)* )? ] => {{
		let hex = $crate::print::HexData::new( $data );
		
		$crate::print!(hex);
	}};
	[ hex: $data: expr $( , $($unk:tt)* )? ] => {{
		let hex = $crate::print::HexData::new( $data );
		
		$crate::print!(hex);
	}};
	
	[] => [];
	[ $($symb:tt)+ ] => {{
		$crate::print::PrintData::print($($symb)*);
	}};
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
