
pub fn crc8_array(arr: &[u8]) -> u8 {
	let mut i = 0u8;
	let max = arr.len() as u8;
	
	let mut rcrc8 = 0;
	while max > i {
		let a = unsafe { *arr.get_unchecked(i as usize) };
		i += 1;
		
		rcrc8 = crc8(rcrc8, a);
	}
	
	rcrc8
}

pub fn crc8(mut crc: u8, mut a: u8) -> u8 {
	for _ in 0..8u8 {
		let b = (crc ^ a) & 0x01;
		crc >>= 1;
		if b == 1 {
			crc ^= 0x8C;
		}
		
		a >>= 1;
	}
	
	crc
}
