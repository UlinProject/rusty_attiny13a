
use crate::delay::{delay_us, delay_ns};

#[inline(always)]
pub fn sleep_inlinealways<const BAUD_SLEEP_US: u64, const BAUD_SLEEP_NS: u64>() {
	if BAUD_SLEEP_US > 0 { // const?!
		delay_us::<BAUD_SLEEP_US>();
	}
	if BAUD_SLEEP_NS > 0 {
		delay_ns::<BAUD_SLEEP_NS>();
	}
}
