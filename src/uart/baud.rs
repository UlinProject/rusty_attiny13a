
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
#[derive(Clone)]
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
#[derive(Clone)]
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
			sub: 0.9, // 0.8
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
		if 0.0 > time {
			time = 0.0;
		}
		
		let us = time as _;
		let ns = (((time * 1000.0)) % 1000.0) as _;
		
		UartBaudTime::new(us, ns)
	}
}
