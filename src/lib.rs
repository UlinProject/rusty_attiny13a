
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]
#![allow(named_asm_labels)]
#![feature(const_trait_impl)]

#![no_std]
extern crate avrd;

/// Progmem utilities for the AVR architectures.
pub mod avr_progmem {
	pub use avr_progmem::*;
}

pub mod int;
pub mod print;
pub mod eeprom;
pub mod volatile;
pub mod osccal;
pub mod delay;
pub mod pio;
pub mod uart;
pub mod bv;
pub mod i2c;
pub mod ssd1306;
pub mod freq;
pub mod cparse;
