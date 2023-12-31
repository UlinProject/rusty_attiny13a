
use crate::delay::{delay_us, delay_ns};

// TODO
// To save memory, we had to sacrifice accuracy.
#[inline(never)] // <<<
pub fn sleep<const SLEEP_US: u64, const SLEEP_NS: u64>() {
	if SLEEP_US > 0 { // const
		delay_us::<SLEEP_US>();
	}
	if SLEEP_NS > 0 {
		delay_ns::<SLEEP_NS>();
	}
}
/*
#[inline]
pub fn doublesleep<const SLEEP_US: u64, const SLEEP_NS: u64>() {
	sleep::<SLEEP_US, SLEEP_NS>();
	sleep::<SLEEP_US, SLEEP_NS>();
}
*/