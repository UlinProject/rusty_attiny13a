
use crate::{pio::Pio, i2c::sleep::sleep};

pub fn read_ack<
	const SDA: Pio,
	const SCL: Pio,
	
	const FULL_IMPULSE_US: u64, const FULL_IMPULSE_NS: u64,
	>() -> bool {
	// current:
	// SCL LOW
	// SDA ~
	
	SDA.input(); // free line for ASC
	//sleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_NS}>();  REQUIRED?
	//sleep::<{QUARTER_IMPULSE_US}, {QUARTER_IMPULSE_NS}>(); // ??
	//sleep::<{QUARTER_IMPULSE_US}, {QUARTER_IMPULSE_NS}>();
	
	SCL.high();
	sleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_NS}>();
	
	
	//sleep::<{QUARTER_IMPULSE_US}, {QUARTER_IMPULSE_NS}>();
	let ask_bit = !SDA.dig_read();
	//ask_bit = true;
	
	SCL.low();
	SDA.output();
	//sleep::<{FULL_IMPULSE_US}, {FULL_IMPULSE_NS}>();  REQUIRED?
	//sleep::<{QUARTER_IMPULSE_US}, {QUARTER_IMPULSE_NS}>();
	//sleep::<{QUARTER_IMPULSE_US}, {QUARTER_IMPULSE_NS}>(); // ??
	//SDA.low(); // ?
	// OUT, exp SDA=LOW
	
	
	// current:
	// SDA_LOW
	// SCL_LOW
	ask_bit
}
