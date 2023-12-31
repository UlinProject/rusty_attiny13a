
use crate::cparse::cstr_to_u64;

/// Current processor frequency, for example: 9.6 MHz - 9_600_000.
pub const CPU_FREQUENCY_HZ: u64 = cstr_to_u64(
	env!("CPU_FREQUENCY_HZ")
);
