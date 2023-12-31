
// #define _BV(bit) (1 << (bit))
#[inline]
pub const fn bv(a: u8) -> u8 {
	1 << a
}

// #define !_BV(bit) (1 << (bit))
#[inline]
pub const fn invers_bv(a: u8) -> u8 {
	!(1 << a)
}