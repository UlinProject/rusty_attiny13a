
use core::marker::ConstParamTy;

#[derive(ConstParamTy, PartialEq, Eq)]
#[derive(Clone, Copy)]
#[repr(u64)]
pub enum SafeUartBaud {
	// For experiment, a little unstable, but it's possible :)))
	//B921600 = 921600,
	
	// For experiment
	B460800 = 460800,
	
	/// see table
	B230400 = 230400,
	/// Standard, can be used.
	B115200 = 115200,
	/// see table
	B57600 = 57600,
	
	// For experiment
	B38400 = 38400,
	
	/// see table
	B9600 = 9600,
	
	// For experiment
	B4800 = 4800,
}

impl SafeUartBaud {
	pub const fn make(self) -> UartBaud {
		UartBaud {
			baud: self as _
		}
	}
}

impl Default for SafeUartBaud {
	#[inline]
	fn default() -> Self {
		Self::B115200
	}
}

//#[derive(ConstParamTy, PartialEq, Eq)]
#[derive(Debug, Clone)]
pub struct UartBaudTime {
	pub us: u8,
	pub ns: u16,
}

impl UartBaudTime {
	#[inline(always)]
	pub const fn new(us: u8, ns: u16) -> Self {
		Self {
			us,
			ns
		}
	}
	
	#[inline(always)]
	pub const fn make_u64(self) -> UartBaudTimeU64 {
		UartBaudTimeU64::new(self)
	}
}

// TODO,
// fix:
// unconstrained generic constant
// try adding a `where` bound using this expression: 
// `where [(); {BAUD_SLEEP.get_us64()}]:
#[derive(ConstParamTy, PartialEq, Eq)]
#[derive(Debug, Clone)]
pub struct UartBaudTimeU64 {
	pub us: u64,
	pub ns: u64,
}

impl UartBaudTimeU64 {
	#[inline(always)]
	pub const fn new(time: UartBaudTime) -> Self {
		Self {
			us: time.us as _,
			ns: time.ns as _
		}
	}
}

pub struct UartBaudCorrection {
	sub: f64,
	add: f64,
	div: f64,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UartBaud {
	baud: u64
}

impl UartBaud {
	#[inline]
	pub const unsafe fn custom(baud: u64) -> Self {
		Self {
			baud
		}
	}
	
	#[inline]
	pub const fn make_rx_time(self) -> UartBaudTime {
		self.make_time(UartBaudCorrection {
			add: 0.0,
			sub: 0.4, // 0.4
			div: 0.0,
		})
	}
	
	#[inline]
	pub const fn make_tx_time(self) -> UartBaudTime {
		self.make_time(UartBaudCorrection {
			// 921600:
			// exp 0,108us
			// 230400:
			// exp 4.34: 
			// +-4.34 - 4.3/4.4
			//
			// 115200:
			// exp 8.68
			// +-8.6 - 8.7
			//
			// 9600:
			// exp 104
			// 104/ +-103.8, 104.6
			add: 0.0,
			sub: 1.2,
			div: 0.0,
		})
	}
	
	#[inline]
	pub const fn make_time(self, correct: UartBaudCorrection) -> UartBaudTime {
		let baud = self.baud as u64;
		let mut time: f64 = (1.0 / (baud as f64)) * 1000000.0;
		
		time += correct.add;
		time -= correct.sub;
		if correct.div != 0.0 {
			time /= correct.div;
		}
		
		let us = time as _;
		let ns = (((time * 1000.0)) % 1000.0) as _;
		
		UartBaudTime::new(us, ns)
	}
	
	/*old..
	#[inline]
	pub const fn make_sd_delaysleep(self) -> u64 {
		// time_fn:
		/*asm!("1: sbiw {i}, 1", // clocks2
				"brne 1b", // clocks 1/2
				i = inout(reg_iw) zero => _,
			)
		*/
		// osccal: 0x48
		// freq: 9.6
		// the values were chosen according to the working code.
		match self {
			Self::B230400 => 7, // 7 - 4.4us (exp: 4.34027778 μs. :()
			Self::B115200 => 10 + (7), // 17 - 8.6us ! (exp: 8.68us)
			Self::B57600 => 21 + (10 + 7), // 38 - 17.4us (exp: 17.4us)
			Self::B9600 => 209 + (21 + 10 + 7), // 247 - 104.0us-105us (exp: 104us) :(
		}
	} */
}
