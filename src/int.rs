
use core::{arch::asm, mem::ManuallyDrop};

/// Creating a zone without interruptions, the zone is determined 
/// by the visibility zone of the variable.
#[repr(transparent)]
pub struct NoIntZone;

impl NoIntZone {
	#[inline(always)]
	pub const unsafe fn new_unchecked() -> Self {
		Self {}
	}
	
	pub fn make() -> Self {
		unsafe {
			asm!("cli");
			
			Self::new_unchecked()
		}
	}
	
	#[inline(always)]
	pub const fn skip_free(self) {
		let _e = ManuallyDrop::new(self);
	}
	
	#[inline(always)]
	pub fn free(self) {}
}

impl Drop for NoIntZone {
	#[inline]
	fn drop(&mut self) {
		unsafe fn _sei() {
			asm!("sei");
		}
		
		unsafe { _sei(); }
	}
}