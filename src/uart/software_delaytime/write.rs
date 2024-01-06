
use core::marker::PhantomData;
use crate::pio::{PioOutLevelState, Pio};
use crate::uart::baud::{UartBaudTimeU64, SafeUartBaud};
use crate::uart::parity::{UartParity, EvenUartParity, calculate_parity, OddUartParity};
use crate::uart::software_delaytime::sleep::sleep_inlinealways;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UartWrite<const N: usize, Parity: UartParity> {
	_pp: PhantomData<Parity>,
	
	arr: [PioOutLevelState; N],
}

impl UartWrite<0, ()> {
	#[inline]
	pub fn build_now() -> UartWrite<10, ()> {
		let current = PioOutLevelState::current();
		
		Self::build(current)
	}
	
	#[inline]
	pub fn build_now_supparity<Parity: UartParity>() -> UartWrite<11, Parity> {
		let current = PioOutLevelState::current();
		
		Self::build_supparity(current)
	}
	
	#[inline]
	pub const fn build_supparity<Parity: UartParity>(current: PioOutLevelState) -> UartWrite<11, Parity> {
		let arr: [PioOutLevelState; 11] = [current; 11];
		let result = UartWrite {
			arr,
			_pp: PhantomData,
		};
		
		result
	}
	
	#[inline]
	pub const fn build(current: PioOutLevelState,) -> UartWrite<10, ()> {
		let arr: [PioOutLevelState; 10] = [current; 10];
		let result = UartWrite {
			arr,
			_pp: PhantomData,
		};
		
		result
	}
}

#[repr(transparent)]
pub struct UartPortWrite<const TXPIO: Pio, const N: usize, Parity: UartParity> {
	builder: UartWrite<N, Parity>
}

impl<const TXPIO: Pio, const N: usize, Parity: UartParity> UartPortWrite<TXPIO, N, Parity> {
	pub const fn cset_byte(mut self, a: u8) -> Self {
		/*
			Interestingly, the loop takes more bytes than writing it directly.
		*/
		const fn update_bitstate<const TXPIO: Pio>(
			i: u8,
			a: u8,
			current: &mut PioOutLevelState,
		) {
			if ((a >> i) & 0x01) != 0 {
				*current = current.c_on::<TXPIO>() // high
			}else {
				*current = current.c_off::<TXPIO>() // low
			}
		}
		
		update_bitstate::<TXPIO>(0, a, &mut self.builder.arr[1]); // 2
		update_bitstate::<TXPIO>(1, a, &mut self.builder.arr[2]); // 3
		update_bitstate::<TXPIO>(2, a, &mut self.builder.arr[3]); // 4
		update_bitstate::<TXPIO>(3, a, &mut self.builder.arr[4]); // 5
		update_bitstate::<TXPIO>(4, a, &mut self.builder.arr[5]); // 6
		update_bitstate::<TXPIO>(5, a, &mut self.builder.arr[6]); // 7
		update_bitstate::<TXPIO>(6, a, &mut self.builder.arr[7]); // 8
		update_bitstate::<TXPIO>(7, a, &mut self.builder.arr[8]); // 9
		self
	}
	
	/*pub fn set_byte(self, a: u8) -> Self {
		let mut sself = self.cset_byte(a);
		if Parity::IS_EXISTS != 0 { // const?!
			{ // 10
				let parity_byte = &mut sself.builder.arr[9];
				*parity_byte = Parity::make_parity(
					a,
					|| parity_byte.c_on::<TXPIO>(), // high_parity
					|| parity_byte.c_off::<TXPIO>(), // low_parity
				); // paritybyte 1
			}
		}
		
		sself
	}*/
}

impl<const TXPIO: Pio, const N: usize> UartPortWrite<TXPIO, N, ()> {
	#[inline]
	pub const fn set_byte(self, a: u8) -> Self {
		self.cset_byte(a)
	}
}

impl<const TXPIO: Pio, const N: usize> UartPortWrite<TXPIO, N, EvenUartParity> {
	pub const fn set_byte(self, a: u8) -> Self {
		let mut sself = self.cset_byte(a);
		
		let parity_byte = &mut sself.builder.arr[9];
		if calculate_parity(a) == 0 {
			*parity_byte = parity_byte.c_on::<TXPIO>();
		} else {
			*parity_byte = parity_byte.c_off::<TXPIO>();
		}
		
		sself
	}
}

impl<const TXPIO: Pio, const N: usize> UartPortWrite<TXPIO, N, OddUartParity> {
	pub const fn set_byte(self, a: u8) -> Self {
		let mut sself = self.cset_byte(a);
		
		let parity_byte = &mut sself.builder.arr[9];
		if calculate_parity(a) == 0 {
			*parity_byte = parity_byte.c_off::<TXPIO>();
		} else {
			*parity_byte = parity_byte.c_on::<TXPIO>();
		}
		
		sself
	}
}


impl<const TXPIO: Pio, const N: usize, Parity: UartParity> UartPortWrite<TXPIO, N, Parity> {
	#[inline(always)]
	pub const fn new(builder: UartWrite<N, Parity>) -> Self {
		Self {
			builder
		}
	}
	
	#[inline(always)]
	pub const fn port<const TXPIO2: Pio>(self) -> UartPortWrite<TXPIO2, N, Parity> {
		self.builder.port::<TXPIO2>()
	}
	
	#[inline(always)]
	pub fn upload_230400(&self) {
		const TX_BAUD: UartBaudTimeU64 = (SafeUartBaud::B230400)
			.make()
			.make_tx_time()
			.make_u64();
		
		self.upload::<{TX_BAUD.us}, {TX_BAUD.ns}>();
	}
	
	#[inline(always)]
	pub fn upload_115200(&self) {
		const TX_BAUD: UartBaudTimeU64 = (SafeUartBaud::B115200)
			.make()
			.make_tx_time()
			.make_u64();
		
		self.upload::<{TX_BAUD.us}, {TX_BAUD.ns}>();
	}
	
	#[inline(always)]
	pub fn upload_57600(&self) {
		const TX_BAUD: UartBaudTimeU64 = (SafeUartBaud::B57600)
			.make()
			.make_tx_time()
			.make_u64();
		
		self.upload::<{TX_BAUD.us}, {TX_BAUD.ns}>();
	}
	
	#[inline(always)]
	pub fn upload_9600(&self) {
		const TX_BAUD: UartBaudTimeU64 = (SafeUartBaud::B9600)
			.make()
			.make_tx_time()
			.make_u64();
		
		self.upload::<{TX_BAUD.us}, {TX_BAUD.ns}>();
	}
	
	#[inline(never)] // It is better to leave it this way to exclude possible optimizations.
	pub fn upload<
		const BAUD_SLEEP_US: u64,
		const BAUD_SLEEP_NS: u64,
	>(&self) {
		let arr: [PioOutLevelState; N] = self.builder.arr;
		
		let mut i = 0u8;
		let len = arr.len() as u8;
		while len > i {
			unsafe { arr.get_unchecked(i as usize).upload_inlinealways(); }
			i += 1;
			
			sleep_inlinealways::<BAUD_SLEEP_US, BAUD_SLEEP_NS>();
		}
	}
}

impl<Parity: UartParity, const N: usize> UartWrite<N, Parity> {
	pub const fn port<const TXPIO: Pio>(mut self) -> UartPortWrite<TXPIO, N, Parity> {
		{ // 0
			let start_byte = &mut self.arr[0]; // ?, exp: const
			
			*start_byte = start_byte.c_off::<TXPIO>();
		}
		{ // 10 or 11
			let pos = self.arr.len()-1; // N-1
			let end_byte = &mut self.arr[pos]; // ?, exp: const
			
			*end_byte = end_byte.c_on::<TXPIO>();
		}
		
		UartPortWrite::new(self)
	}
	
	#[inline]
	pub const unsafe fn with_prep_port<const TXPIO: Pio>(self) -> UartPortWrite<TXPIO, N, Parity> {
		UartPortWrite::new(self)
	}
}