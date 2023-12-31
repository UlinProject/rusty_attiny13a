
use crate::{pio::{PioInLevelState, Pio}, uart::software_delaytime::sleep::sleep_inlinealways};

#[inline(never)]
pub fn uart_read<
	const BAUD_SLEEP_US: u64, 
	const BAUD_SLEEP_NS: u64, 
	const RXPIO: Pio, 
	
	const COUNT: usize
>(mut next: impl FnMut(u8, u8)) {
	{ // TODO
		// const_check:
		//
		// If you are reading this line, then you have exceeded the allowed 
		// number of bytes available for reading. I couldn't have achieved 
		// a better result.
		const _CHECK_ARR: [(); 19] = [(); 19];
		let _check_max = _CHECK_ARR[COUNT as usize];
	}
	let max = COUNT as u8;
	let mut result;
	
	let mut i = 0u8;
	while max > i {
		result = 0u8;
		let mut ci = 0u8;
		let maxci = 8;
		
		while PioInLevelState::c_static_is::<RXPIO>() {} // start
		
		while maxci > ci {
			if PioInLevelState::c_static_is::<RXPIO>() {
				result |= 0x80;
			}
			result >>= 1;
			ci += 1;
			
			sleep_inlinealways::<BAUD_SLEEP_US, BAUD_SLEEP_NS>();
		}
		next(i, result);
		i += 1;
		
		while !PioInLevelState::c_static_is::<RXPIO>() {} // end
	}
}

