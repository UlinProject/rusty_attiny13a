
use crate::{i2c::sleep::sleep, pio::Pio};

pub fn start_event<
	const SDA: Pio,
	const SCL: Pio,
	
	const FULL_IMPULSE_US: u64, const FULL_IMPULSE_NS: u64,
	//const DOUBLE_IMPULSE_US: u64, const DOUBLE_IMPULSE_NS: u64,
	//const QUARTER_IMPULSE_US: u64, const QUARTER_IMPULSE_NS: u64,
>() {
	// current:
	// SDA: HIGH
	// SCL: HIGH
	//
	// ...
	SDA.low();
	sleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_US}>();
	// doublesleep or sleep? REQUIRED?
	SCL.low();
	//sleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_NS}>(); REQUIRED?
	//sleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_NS}>(); REQUIRED?
	
	
	
	// end_current:
	// SDA: LOW
	// SCL: LOW
}

pub fn stop_event<
	const SDA: Pio,
	const SCL: Pio,

	const FULL_IMPULSE_US: u64, const FULL_IMPULSE_NS: u64,
	//const DOUBLE_IMPULSE_US: u64, const DOUBLE_IMPULSE_NS: u64,
	//const QUARTER_IMPULSE_US: u64, const QUARTER_IMPULSE_NS: u64,
>() {
	// STOP
	// SDA: LOW
	// SCL: LOW
	//
	// ...
	
	//sleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_NS}>(); REQUIRED?
	SCL.high();
	sleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_US}>();
	SDA.high();
	//doublesleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_US}>(); REQUIRED?
	
	// END:
	// SDA: HIGH
	// SCL: HIGH
}

pub fn insert_bit<
	const SDA: Pio,
	const SCL: Pio,
	
	const FULL_IMPULSE_US: u64, const FULL_IMPULSE_NS: u64,
>(is_high: bool) {
	// if start
	// then scl_low+sda_low
	
	// current:
	// SCL: LOW/STAND
	//
	
	if is_high {
		SDA.high();
	}else {
		SDA.low();
	}
	
	//sleep::<{QUARTER_IMPULSE_US}, {QUARTER_IMPULSE_NS}>();
	SCL.high();
	sleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_NS}>(); // ????? REQUIRED?
	SCL.low(); // ALWAYS END SCL LOW.
	sleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_NS}>(); // ????? REQUIRED?
}