
use avrd::current::OSCCAL;
use crate::{eeprom::{EepromAddr, eeprom_read}, volatile, int::NoIntZone};

/// Download and set the internal frequency calibration bits from the eeprom. 
/// 
/// (Accordingly, you should have previously calibrated this microcontroller 
/// and written the calibration result to the EEPROM at address 0.)
pub fn loadosccal_from_eeprom(int: &NoIntZone) {
	let cal = eeprom_read(int, EepromAddr::OSCCAL_ADDR); // 0x47/0x48 good
		
	if cal < 0x7F {
		unsafe {
			set_osccal(cal);
		}
		
		//print!(@progmem: b"New osccal val: ");
		//println!(@hex: cal);
	}
}

#[inline]
pub unsafe fn set_osccal(osccal: u8) {
	volatile!(*OSCCAL = osccal);
}
