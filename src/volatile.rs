
/// A pointer that designates any interaction with the pointer as volatile.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct VolatilePtr {
	ptr: *mut u8
}

impl From<*mut u8> for VolatilePtr {
	#[inline(always)]
	fn from(value: *mut u8) -> Self {
		Self::new(value)
	}
}

impl VolatilePtr {
	#[inline(always)]
	pub const fn new(ptr: *mut u8) -> Self {
		Self {
			ptr
		}
	}
	
	#[inline(always)]
	pub unsafe fn is(&self) -> bool {
		self.read() != 0
	}
	
	#[inline(always)]
	pub unsafe fn read(&self) -> u8 {
		self.ptr.read_volatile()
	}
	
	#[inline(always)]
	pub unsafe fn bv_read(&self) -> u8 {
		crate::bv::bv(self.read())
	}
	
	#[inline(always)]
	pub unsafe fn invers_bv_read(&self) -> u8 {
		crate::bv::invers_bv(self.read())
	}
	
	#[inline(always)]
	pub unsafe fn write(&self, a: u8) {
		self.ptr.write_volatile(a)
	}
}

#[macro_export]
macro_rules! volatile {
	[
		_bv(*$a: ident)
	] => {{
		unsafe {
			crate::volatile::VolatilePtr::new($a).bv_read()
		}
	}};
	[
		_invers_bv(*$a: ident)
	] => {{
		unsafe {
			crate::volatile::VolatilePtr::new($a).invers_bv_read()
		}
	}};
	[
		*$a: ident
	] => {{
		unsafe {
			crate::volatile::VolatilePtr::new($a).read()
		}
	}};
	[
		*$a: ident |= $e: expr $(; $($all:tt)*)?
	] => {{
		#[allow(unused_unsafe)]
		unsafe {
			let ptr = crate::volatile::VolatilePtr::new($a);
			
			let mut new_value = ptr.read();
			new_value |= $e;
			ptr.write(new_value)
		}
		$(
			$crate::volatile! {
				$($all)*
			}
		)?
	}};
	[
		*$a: ident = $e: expr $(; $($all:tt)*)?
	] => {{
		#[allow(unused_unsafe)]
		unsafe {
			crate::volatile::VolatilePtr::new($a).write($e);
		}
		$(
			$crate::volatile! {
				$($all)*
			}
		)?
	}};
	/*[ $unk:tt $($unk2:tt)* ] => [
		$unk
		
		$crate::volatile! {
			$($unk2)*
		}
	];*/
	[] => []
}
