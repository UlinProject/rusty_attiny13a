
pub mod write;
pub mod read;
pub mod sleep;

use crate::pio::Pio;

/// Creating a TX port for recording via UART.
pub fn init_tx_port<const TXPIO: Pio>() {
	TXPIO.output();
	TXPIO.high();
}

/// Creating an RX port for reading via UART.
pub fn init_rx_port<const RXPIO: Pio>() {
	RXPIO.input_pullup(); // ?!
}
