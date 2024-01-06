
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
	[ $($ty: ty),* $(,)? ] => {
		$(
			impl BvNum for $ty {
				const ONE: Self = 1;
			}
		)*
	};
}

_bv_num! {
	u8, u16, u32, u64,
	i8, i16, i32, i64,
}
