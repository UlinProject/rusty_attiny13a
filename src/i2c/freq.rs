
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct I2CFreq {
	freq: u64
}

impl I2CFreq {
	#[inline(always)]
	pub const fn k100() -> Self {
		Self {
			freq: 100_000 //hz
		}
	}
	
	#[inline(always)]
	pub const fn k200() -> Self {
		Self {
			freq: 200_000 //hz
		}
	}
	
	/*#[inline(always)]
	pub const fn k300() -> Self {
		Self {
			freq: 300_000 //hz
		}
	}*/
	
	/// TODO, accuraty :(, output +-344-355khz
	#[inline(always)]
	pub const fn k400() -> Self {
		Self {
			freq: 400_000 //khz
		}
	}
	
	/// TODO, accuraty :(, output +-520-555khz
	#[inline(always)]
	pub const fn k800() -> Self {
		Self {
			freq: 800_000 //khz
		}
	}
	
	#[inline(always)]
	pub const fn m1() -> Self {
		Self {
			freq: 1_000_000 //1mhz
		}
	}
}

pub struct I2CFreqCorrection {
	sub: f64,
	add: f64,
	div: f64,
	mul: f64,
}

#[derive(Clone)]
pub struct I2CFreqTimeU64 {
	pub full_impulse_us: u64,
	pub full_impulse_ns: u64,
	
	pub double_impulse_us: u64,
	pub double_impulse_ns: u64,
	
	pub quarter_impulse_us: u64,
	pub quarter_impulse_ns: u64,
}

impl I2CFreq {
	#[inline]
	pub const fn make_sync_time(self) -> I2CFreqTimeU64 {
		/*
			TODO
			to save memory, there are no settings yet.
		*/
		let addcorrect_freq_us: u64 = 0;
		let addcorrect_freq_ns: u64 = 0;
		
		let mulcorrect_freq_us: u64 = 0;
		let mulcorrect_freq_ns: u64 = 0;
		
		let (full_impulse_us, full_impulse_ns) = self.make_time(I2CFreqCorrection {
			add: 0.0,
			sub: 0.0,
			div: 0.0,
			mul: 0.0,
		});
		
		let (double_impulse_us, double_impulse_ns) = self.make_time(I2CFreqCorrection {
			add: 0.0,
			sub: 0.0,
			div: 0.0,
			mul: 2.0,
		});
		
		let (quarter_impulse_us, quarter_impulse_ns) = self.make_time(I2CFreqCorrection {
			add: 0.0,
			sub: 0.0,
			div: 4.0,
			mul: 0.0,
		});
		
		I2CFreqTimeU64 {
			full_impulse_us: (full_impulse_us + addcorrect_freq_us) - mulcorrect_freq_us,
			full_impulse_ns: (full_impulse_ns + addcorrect_freq_ns) - mulcorrect_freq_ns,
			
			double_impulse_us: (double_impulse_us + addcorrect_freq_us) - mulcorrect_freq_us,
			double_impulse_ns: (double_impulse_ns + addcorrect_freq_ns) - mulcorrect_freq_ns,
			
			quarter_impulse_us: (quarter_impulse_us + addcorrect_freq_us) - mulcorrect_freq_us,
			quarter_impulse_ns: (quarter_impulse_ns + addcorrect_freq_ns) - mulcorrect_freq_ns,
		}
	}
	
	#[inline]
	pub const fn make_time(self, correct: I2CFreqCorrection) -> (u64, u64) {
		let freq = self.freq;
		
		let mut time: f64 = (1.0 / (freq as f64)) * 1000000.0;
		
		time += correct.add;
		time -= correct.sub;
		if correct.div != 0.0 {
			time /= correct.div;
		}
		if correct.mul != 0.0 {
			time *= correct.mul;
		}
		if 0.0 > time {
			time = 0.0;
		}
		
		// TODO
		let us = time as u64 as f64;
		let mut ns = (time-us) * 10.0;
		let mut i = 0;
		while (ns - (ns as u64 as f64)) > 0.0 {
			ns *= 10.0;
			
			i += 1;
			if i > 4 { // TODO, safe flash :(
				break;
			}
		}
		
		(us as _, ns as _)
	}
}
