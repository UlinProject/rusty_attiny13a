
pub mod addr;
pub mod freq;
pub mod sleep;
pub mod events;
pub mod ack;
pub mod generic;

use crate::i2c::ack::read_ack;
use crate::i2c::addr::I2CAddr;
use crate::i2c::freq::I2CFreq;
use crate::i2c::freq::I2CFreqTimeU64;
use crate::i2c::generic::I2CGenMaster;
use crate::i2c::generic::I2CGenTransaction;
use crate::pio::Pio;
use sleep::sleep;

fn insert_bit<
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

#[repr(transparent)]
pub struct I2CMaster<
	const SDA: Pio,
	const SCL: Pio,
	
	const FULL_IMPULSE_US: u64, const FULL_IMPULSE_NS: u64,
> {}

#[repr(transparent)]
pub struct I2CWriteTransaction<
	const SDA: Pio, 
	const SCL: Pio,
	
	const FULL_IMPULSE_US: u64, const FULL_IMPULSE_NS: u64,
> {
	addr: I2CAddr,
}

impl<
	const SDA: Pio, 
	const SCL: Pio,

	const FULL_IMPULSE_US: u64, const FULL_IMPULSE_NS: u64,
> I2CGenTransaction for I2CWriteTransaction<
	SDA, 
	SCL, 

	FULL_IMPULSE_US, FULL_IMPULSE_NS,
> {
	#[inline]
	fn write(&self, data: u8) -> bool {
		I2CWriteTransaction::write(self, data)
	}
	
	#[inline]
	unsafe fn stop(&self) {
		I2CWriteTransaction::_stop(&self)
	}
}

impl<
	const SDA: Pio,
	const SCL: Pio,
	
	const FULL_IMPULSE_US: u64, const FULL_IMPULSE_NS: u64,
> I2CWriteTransaction<
	SDA, 
	SCL, 
	
	FULL_IMPULSE_US, FULL_IMPULSE_NS,
> {
	pub fn write(
		&self,
		mut data: u8,
		
	) -> bool {
		let ack_bit;
		
		// start:
		// SDA+SCL: LOW
		//
		
		{ // DATA WRITE
			let mut i = 0u8;
			while 8 > i {
				insert_bit::<
					SDA, SCL, 
					
					FULL_IMPULSE_US, FULL_IMPULSE_NS,
				>((data & 0x80) != 0);
				
				data <<= 1;
				i += 1;
			}
			// 7bitaddr(pseudo8bit) + start(data <<= 1;), 8 bit always == 0 then:
			// always write mode!
			//
		}
		
		// current:
		// SCL LOW
		// SDA ~
		// ...
		{ // ACK
			ack_bit = read_ack::<SDA, SCL, FULL_IMPULSE_US, FULL_IMPULSE_NS>();
			
			// current:
			// SDA_LOW
			// SCL_LOW
		}
		
		ack_bit
	}
	
	#[inline]
	fn _start(&self) {
		events::start_event::<
			SDA, SCL, 
				
			FULL_IMPULSE_US, FULL_IMPULSE_NS,
		>();
	}
	
	#[inline]
	fn _stop(&self) {
		events::stop_event::<
			SDA, SCL,
			
			FULL_IMPULSE_US, FULL_IMPULSE_NS,
		>()
	}
}

pub const K400: I2CFreqTimeU64 = I2CFreq::k400().make_sync_time();
pub const K400_FULL_NS: u64 = K400.full_impulse_ns;
pub const K400_FULL_US: u64 = K400.full_impulse_us;
/*pub const K400_DOUBLE_NS: u64 = K400.double_impulse_ns;
pub const K400_DOUBLE_US: u64 = K400.double_impulse_us;
pub const K400_QUARTER_NS: u64 = K400.quarter_impulse_ns;
pub const K400_QUARTER_US: u64 = K400.quarter_impulse_us;*/

pub const K100: I2CFreqTimeU64 = I2CFreq::k100().make_sync_time();
pub const K100_FULL_NS: u64 = K100.full_impulse_ns;
pub const K100_FULL_US: u64 = K100.full_impulse_us;
/*pub const K100_DOUBLE_NS: u64 = K100.double_impulse_ns;
pub const K100_DOUBLE_US: u64 = K100.double_impulse_us;
pub const K100_QUARTER_NS: u64 = K100.quarter_impulse_ns;
pub const K100_QUARTER_US: u64 = K100.quarter_impulse_us;*/

pub const K200: I2CFreqTimeU64 = I2CFreq::k200().make_sync_time();
pub const K200_FULL_NS: u64 = K200.full_impulse_ns;
pub const K200_FULL_US: u64 = K200.full_impulse_us;
/*pub const K200_DOUBLE_NS: u64 = K200.double_impulse_ns;
pub const K200_DOUBLE_US: u64 = K200.double_impulse_us;
pub const K200_QUARTER_NS: u64 = K200.quarter_impulse_ns;
pub const K200_QUARTER_US: u64 = K200.quarter_impulse_us;*/

/*pub const K300: I2CFreqTimeU64 = I2CFreq::k300().make_sync_time();
pub const K300_FULL_NS: u64 = K300.full_impulse_ns;
pub const K300_FULL_US: u64 = K300.full_impulse_us;
pub const K300_DOUBLE_NS: u64 = K300.double_impulse_ns;
pub const K300_DOUBLE_US: u64 = K300.double_impulse_us;
pub const K300_QUARTER_NS: u64 = K300.quarter_impulse_ns;
pub const K300_QUARTER_US: u64 = K300.quarter_impulse_us;*/

pub const K800: I2CFreqTimeU64 = I2CFreq::k800().make_sync_time();
pub const K800_FULL_NS: u64 = K800.full_impulse_ns;
pub const K800_FULL_US: u64 = K800.full_impulse_us;
/*pub const K800_DOUBLE_NS: u64 = K800.double_impulse_ns;
pub const K800_DOUBLE_US: u64 = K800.double_impulse_us;
pub const K800_QUARTER_NS: u64 = K800.quarter_impulse_ns;
pub const K800_QUARTER_US: u64 = K800.quarter_impulse_us;*/

impl<const SDA: Pio, const SCL: Pio> I2CMaster<SDA, SCL, 0, 0> {
	// TODO, accuraty:(
	#[inline]
	pub fn init_400khz() -> I2CMaster<
		SDA,
		SCL,
		
		{K400_FULL_US}, {K400_FULL_NS},
		//{K400_DOUBLE_US}, {K400_DOUBLE_NS},
		//{K400_QUARTER_US}, {K400_QUARTER_NS},
	> {
		I2CMaster::init()
	}
	
	// +- normal
	#[inline]
	pub fn init_100khz() -> I2CMaster<
		SDA,
		SCL,
		
		{K100_FULL_US}, {K100_FULL_NS},
		//{K100_DOUBLE_US}, {K100_DOUBLE_NS},
		//{K100_QUARTER_US}, {K100_QUARTER_NS},
	> {
		I2CMaster::init()
	}
	
	// +- normal
	#[inline]
	pub fn init_200khz() -> I2CMaster<
		SDA,
		SCL,
		
		{K200_FULL_US}, {K200_FULL_NS},
		//{K200_DOUBLE_US}, {K200_DOUBLE_NS},
		//{K200_QUARTER_US}, {K200_QUARTER_NS},
	> {
		I2CMaster::init()
	}
	
	/*#[inline] more more flash + invalid 3khz:(
	pub fn init_300khz() -> I2CMaster<
		SDA,
		SCL,
		
		{K300_FULL_US}, {K300_FULL_NS},
		{K300_DOUBLE_US}, {K300_DOUBLE_NS},
		{K300_QUARTER_US}, {K300_QUARTER_NS},
	> {
		I2CMaster::init()
	}*/
	
	#[inline]
	pub fn init_800khz() -> I2CMaster<
		SDA,
		SCL,
		
		{K800_FULL_US}, {K800_FULL_NS},
		//{K800_DOUBLE_US}, {K800_DOUBLE_NS},
		//{K800_QUARTER_US}, {K800_QUARTER_NS},
	> {
		I2CMaster::init()
	}
	
	/// Elimination of all possible timings, sending data via 
	/// the bus at the maximum allowable speed of the microcontroller.
	/// 
	/// may damage data or display (usually not).
	#[inline]
	pub unsafe fn init_0khz() -> I2CMaster<
		SDA,
		SCL,
		
		0, 0,
		//0, 0,
		//0, 0,
	> {
		I2CMaster::init()
	}
}

impl<
	const SDA: Pio, const SCL: Pio,
		
	const FULL_IMPULSE_US: u64, const FULL_IMPULSE_NS: u64,
> I2CGenMaster for I2CMaster<
	SDA, 
	SCL, 

	FULL_IMPULSE_US, FULL_IMPULSE_NS,
> {
	type Transaction = I2CWriteTransaction<
		SDA, SCL,
		
		FULL_IMPULSE_US, FULL_IMPULSE_NS,
	>;
	#[inline]
	fn scan(&self, success: impl FnMut(I2CAddr), noexists: impl FnMut(I2CAddr)) {
		I2CMaster::scan(self, success, noexists)
	}
	
	#[inline]
	fn is_exists(&self, addr: I2CAddr) -> bool {
		I2CMaster::is_exists(self, addr)
	}
	
	#[inline]
	fn safestart(&self, addr: I2CAddr,
		transac_confired: impl FnOnce(
			&<Self>::Transaction
		),
	) -> bool {
		I2CMaster::safestart(self, addr, transac_confired)
	}
	
	#[inline]
	unsafe fn start(
		&self,
		addr: I2CAddr
	) -> (bool, Self::Transaction) {
		I2CMaster::start(self, addr)
	}
}

impl<
	const SDA: Pio, const SCL: Pio,
	
	const FULL_IMPULSE_US: u64, const FULL_IMPULSE_NS: u64,
> I2CMaster<
	SDA, 
	SCL, 
	
	FULL_IMPULSE_US, FULL_IMPULSE_NS,
> {
	#[inline]
	pub fn init() -> Self {
		SDA.output();
		SCL.output();
	
		//SDA.high(); see start/stop
		//SCL.high();;
		
		// It's strange, but until I do a manual start/stop, 
		// the bus only works after a transaction.
		events::start_event::<
			SDA, SCL,
			
			FULL_IMPULSE_US, FULL_IMPULSE_NS,
		>();
		events::stop_event::<
			SDA, SCL,
			
			FULL_IMPULSE_US, FULL_IMPULSE_NS,
		>();
		
		Self {}
	}
	
	#[inline(always)]
	pub const fn gen(self) -> impl I2CGenMaster {
		self
	}
	
	///
	/// ```rust
	/// i2cmaster.scan(|addr| {
	///		print!(b"Exists: ");
	///		print!(@hex: addr);
	///		print!(b"\r\n");
	///	}, |_addr| {
	///		// no_exists
	///	});
	/// ```
	pub fn scan(
		&self, 
		mut success: impl FnMut(I2CAddr),
		mut noexists: impl FnMut(I2CAddr)
	) {
		const START: I2CAddr = I2CAddr::start_addrwrite();
		const END: I2CAddr = I2CAddr::end_addrwrite();
		
		let mut start_addr = START;
		let end_addr = END;
		while end_addr > start_addr {
			{ // i2c
				let (result, transaction) = unsafe {
					self.start(start_addr)
				};
				if result {
					success(start_addr);
				}else {
					noexists(start_addr);
				}
				transaction._stop();
			}
			
			unsafe {
				start_addr = start_addr.next();
			}
		}
	}
	
	unsafe fn start(
		&self,
		addr: I2CAddr
	) -> (bool, I2CWriteTransaction<
		SDA, SCL,
		
		FULL_IMPULSE_US, FULL_IMPULSE_NS,
	>) {
		let transaction = I2CWriteTransaction {
			addr
		};
		
		transaction._start();
		
		let result = transaction.write( // write addr
			transaction.addr.read(),
		);
		
		(result, transaction)
	}
	
	pub fn safestart(
		&self,
		addr: I2CAddr,
		transac_confired: impl FnOnce(
			&I2CWriteTransaction<
				SDA, SCL,
				
				FULL_IMPULSE_US, FULL_IMPULSE_NS,
			>
		),
	) -> bool {
		let (result, transaction) = unsafe {
			self.start(addr)
		};
		if result {
			transac_confired(&transaction);
		}
		transaction._stop();
		result
	}
	
	#[inline]
	pub fn is_exists(&self, addr: I2CAddr) -> bool {
		let (result, transaction) = unsafe {
			self.start(addr)
		};
		transaction._stop();
		result
	}
}
