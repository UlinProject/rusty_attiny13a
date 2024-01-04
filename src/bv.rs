
use core::ops::{Shl, Not};

// #define _BV(bit) (1 << (bit))
#[inline]
pub const fn bv<T>(a: T) -> T where T: BvNum + Shl<Output = T> {
	T::ONE << a
}

// #define !_BV(bit) (1 << (bit))
#[inline]
pub const fn invers_bv<T>(a: T) -> T where T: BvNum + Shl<Output = T> + Not<Output = T> {
	!(T::ONE << a)
}

pub trait BvNum {
	const ONE: Self;
}

macro_rules! _bv_num {
	[ $([$ty: ty: $te: expr]);* $(;)? ] => {
		$(
			impl BvNum for $ty {
				const ONE: Self = $te;
			}
		)*
	};
}

_bv_num! {
	[u8: 1];
	[u16: 1];
	[u32: 1];
	[u64: 1];
	
	[i8: 1];
	[i16: 1];
	[i32: 1];
	[i64: 1];
}
