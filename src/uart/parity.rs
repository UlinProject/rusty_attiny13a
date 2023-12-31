
pub trait UartParity: Clone + Copy {
	const IS_EXISTS: usize;
	
	fn make_parity<R>(
		byte: u8,
		event_add_high_bit: impl FnOnce() -> R, 
		event_add_low_bit: impl FnOnce() -> R
	) -> R;
}


pub type SkipUartParity = ();
impl UartParity for SkipUartParity {
	const IS_EXISTS: usize = 0;
	
	#[inline]
	fn make_parity<R>(
		_byte: u8,
		
		_event_add_high_bit: impl FnOnce() -> R, 
		_event_add_low_bit: impl FnOnce() -> R
	) -> R {
		_event_add_low_bit()
	}
}

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

#[derive(Clone, Copy)]
pub enum EvenUartParity {}

impl UartParity for EvenUartParity {
	const IS_EXISTS: usize = 1;
	
	#[inline]
	fn make_parity<R>(
		byte: u8,
		
		event_add_high_bit: impl FnOnce() -> R, 
		event_add_low_bit: impl FnOnce() -> R
	) -> R {
		if calculate_parity(byte) == 0 {
			event_add_high_bit()
		} else {
			event_add_low_bit()
		}
	}
}


#[derive(Clone, Copy)]
pub enum OddUartParity {}

impl UartParity for OddUartParity {
	const IS_EXISTS: usize = 1;
	
	#[inline]
	fn make_parity<R>(
		byte: u8,
		
		event_add_high_bit: impl FnOnce() -> R, 
		event_add_low_bit: impl FnOnce() -> R
	) -> R {
		if calculate_parity(byte) == 0 {
			event_add_low_bit()
		} else {
			event_add_high_bit()
		}
	}
}