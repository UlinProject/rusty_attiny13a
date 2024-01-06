
use crate::cparse::cstr_to_u64;

/// Current processor frequency, 
/// 
/// (for example: 9.6 MHz - 9_600_000.)
pub const CPU_FREQUENCY_HZ: u64 = match option_env!("CPU_FREQUENCY_HZ") {
	None => match option_env!("AVR_CPU_FREQUENCY_HZ") {
		None => 9_600_000,
		Some(freq) => cstr_to_u64(freq),
	},
	Some(freq) => cstr_to_u64(freq)
};
