
pub trait UartParity: Clone + Copy {}

/// Do not generate parity bit when writing via uart.
pub type SkipUartParity = ();
impl UartParity for SkipUartParity {}

/// Creating a parity bit (used for writing via uart)
pub const fn calculate_parity(val: u8) -> u8 {
	/*let in0: u8 = val;
	let mut out0: u8;
	unsafe {
		asm!(
			"mov {out0}, {in0}",
			"swap {in0}",
			"eor {in0}, {out0}",
			"mov {out0}, {in0}",
			"lsr {in0}",
			"lsr {in0}",
			"eor {in0}, {out0}",
			in0 = in(reg) in0,
			out0 = out(reg) out0,
		);
		
		((out0 + 1) >> 1) & 1
	}*/
	let mut val = val as u16;
	val ^= val >> 8;
	val ^= val >> 4;
	val ^= val >> 2;
	val ^= val >> 1;
	
	(!val & 1) as u8
}

/// Even parity bit for writing via uart.
#[derive(Clone, Copy)]
pub enum EvenUartParity {}
impl UartParity for EvenUartParity {}

/// Odd parity bit for writing via uart.
#[derive(Clone, Copy)]
pub enum OddUartParity {}
impl UartParity for OddUartParity {}
