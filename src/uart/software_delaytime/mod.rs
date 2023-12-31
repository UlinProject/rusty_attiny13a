
pub mod write;
pub mod read;
pub mod sleep;

use crate::pio::Pio;

pub fn init_tx_port<const TXPIO: Pio>() {
	TXPIO.output();
	TXPIO.high();
}

pub fn init_rx_port<const RXPIO: Pio>() {
	RXPIO.input_pullup(); // ?!
}