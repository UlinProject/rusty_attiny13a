
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]
#![feature(effects)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]

#![no_std]
pub extern crate avrd;
pub extern crate avr_progmem;

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
pub mod crc;
