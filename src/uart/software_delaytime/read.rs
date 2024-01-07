
use crate::{pio::{PioInLevelState, Pio}, uart::software_delaytime::sleep::sleep_inlinealways};

pub struct UartRead {
	pub(crate) i: u8,
	pub(crate) data: u8,
}

/// Batch reading UART. COUNT - the number of bytes required to read, 
/// FnMut is called every time a new byte is received, the result 
/// determines the success of the operation.
///
/// (FnMut is latency sensitive).
#[inline(never)]
pub fn uart_read<
	const BAUD_SLEEP_US: u64, 
	const BAUD_SLEEP_NS: u64, 
	const RXPIO: Pio, 
	
	const COUNT: usize
>(mut next: impl FnMut(UartRead)) -> bool {
	let mut result;
		
	let mut i = 0u8;
	while (COUNT as u8) > i {
		result = 0u8;
		let mut ci = 0u8;
		let maxci = 8u8;
		
		while PioInLevelState::c_static_is::<RXPIO>() {} // start
		
		while maxci > ci {
			sleep_inlinealways::<BAUD_SLEEP_US, BAUD_SLEEP_NS>();
			
			result >>= 1;
			if PioInLevelState::c_static_is::<RXPIO>() {
				result |= 0x80;
			}
			ci += 1;
		}
		next(UartRead {
			i,
			data: result,
		});
		i += 1;
		
		sleep_inlinealways::<BAUD_SLEEP_US, BAUD_SLEEP_NS>();
		let stop = RXPIO.dig_read();
		if stop != true {
			return false;
		}
	}
	
	true
}

/// Reading one byte from uart. 
/// 
/// (important, don't use it, if you need to read two or more bytes, 
/// use another function for that) (this function is very economical 
/// for flash memory and only).
#[inline(never)]
pub fn uart_oneread<
	const BAUD_SLEEP_US: u64,
	const BAUD_SLEEP_NS: u64,
	const RXPIO: Pio,
>() -> Option<u8> {
	let mut result = 0u8;
	let mut ci = 0u8;
	let maxci = 8u8;
	
	while PioInLevelState::c_static_is::<RXPIO>() {} // start
	
	while maxci > ci {
		sleep_inlinealways::<BAUD_SLEEP_US, BAUD_SLEEP_NS>();
		
		result >>= 1;
		if PioInLevelState::c_static_is::<RXPIO>() {
			result |= 0x80;
		}
		ci += 1;
	}
	
	sleep_inlinealways::<BAUD_SLEEP_US, BAUD_SLEEP_NS>();
	let stop = RXPIO.dig_read();
	match stop {
		true => Some(result),
		false => None
	}
}
