
///
/// Convert constant strings to numbers.
/// (for use in constant environments only)
pub const fn cstr_to_u64(str: &'static str) -> u64 {
	// check_symb
	let mut i = 0usize;
	let max = str.len();
	let array = str.as_bytes();
	
	let mut num = 0;
	while max > i {
		let a = array[i];
		i += 1;
		
		if a == b' ' || a == b'_' {
			continue;
		}
		if a > b'9' || a < b'0' {
			return 0;
		}
		
		num = (num * 10) + (a - b'0') as u64;
	}
	
	num
}

