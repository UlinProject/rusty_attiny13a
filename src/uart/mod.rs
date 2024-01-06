
use core::mem::MaybeUninit;

use crate::{pio::Pio, uart::{baud::{UartBaud, SafeUartBaud, UartBaudTimeU64}, parity::EvenUartParity, software_delaytime::read::{uart_read, uart_oneread}}, cparse::cstr_to_u64};

pub mod baud;
pub mod parity;
pub mod software_delaytime;

///
/// uart_write!(@B115200, a, PB0, PB4);
#[macro_export]
macro_rules! uart_write {
	[ @$speed:tt, $parity:tt, $a: ident, $port:tt $(, $port2:tt)* $(,)? ] => {{
		use crate::uart::software_delaytime::write::UartWrite;
		use crate::uart::baud::UartBaudTimeU64;
		
		let write = UartWrite::build_now_supparity::<$parity>()
			.port::<{$port}>()
			.set_byte($a);
		
		$( let write = write.port::<{$port2}>().set_byte($a); )*
		
		const TX_BAUD: UartBaudTimeU64 = ($speed).make_tx_time().make_u64();
		write.upload::<{TX_BAUD.us}, {TX_BAUD.ns}>();
	}};
}

pub const UART_BAUD: UartBaud = {
	match option_env!("UART_BAUD") {
		None => SafeUartBaud::B115200.make(),
		Some(cstrbaud) => {
			let cbaud = cstr_to_u64(cstrbaud);
			
			unsafe {
				UartBaud::custom(cbaud)
			}
		}
	}
};
pub type DefUartParity = EvenUartParity;
pub const UART_TX_PIN: Pio = Pio::PB0;
pub const UART_RX_PIN: Pio = Pio::PB1;
//pub const UART_TX2_PIN: Pio = Pio::PB4;
const RX_BAUD: UartBaudTimeU64 = (UART_BAUD).make_rx_time().make_u64();

#[inline(never)]
pub fn serial_write_byte(a: u8) {
	uart_write!(@UART_BAUD, DefUartParity, a, UART_TX_PIN/*, UART_TX2_PIN */);
}

//#[inline(never)] see uart_read
pub fn serial_read<const N: usize>() -> Option<[u8; N]> {
	// We reduce the cost of initialization.
	let mut array: [MaybeUninit<u8>; N] = {
		MaybeUninit::uninit_array::<N>()
	};
	
	match uart_read::<{RX_BAUD.us}, {RX_BAUD.ns}, {UART_RX_PIN}, {N}>(|a| {
		unsafe { *array.get_unchecked_mut(a.i as usize) = MaybeUninit::new(a.data); }
	}) {
		true => Some(unsafe { MaybeUninit::array_assume_init(array) }),
		false => None,
	}
}

/// Reading one byte from uart. 
/// (important, don't use it, if you need to read two or more bytes, 
/// use another function for that) (this function is very economical 
/// for flash memory and only).
pub fn serial_oneread() -> Option<u8> {
	uart_oneread::<{RX_BAUD.us}, {RX_BAUD.ns}, {UART_RX_PIN}>()
}

pub fn serial_init() {
	software_delaytime::init_tx_port::<{UART_TX_PIN}>();
	software_delaytime::init_rx_port::<{UART_RX_PIN}>();
	//software_delaytime::init_tx_port::<{UART_TX2_PIN}>();
}
