
use crate::i2c::addr::I2CAddr;

pub trait I2CGenMaster {
	type Transaction: I2CGenTransaction;
	
	fn scan(&self, success: impl FnMut(I2CAddr), noexists: impl FnMut(I2CAddr));
	fn is_exists(&self, addr: I2CAddr) -> bool;
	
	fn safestart(&self, addr: I2CAddr,
		transac_confired: impl FnOnce(
			&Self::Transaction
		),
	) -> bool;
	
	unsafe fn start(
		&self,
		addr: I2CAddr
	) -> (bool, Self::Transaction);
}

impl<'a, T> I2CGenMaster for &'a T where T: I2CGenMaster {
	type Transaction = T::Transaction;

	#[inline(always)]
	fn scan(&self, success: impl FnMut(I2CAddr), noexists: impl FnMut(I2CAddr)) {
		(*self).scan(success, noexists)
	}
	
	#[inline(always)]
	fn is_exists(&self, addr: I2CAddr) -> bool {
		(*self).is_exists(addr)
	}

	#[inline(always)]
	fn safestart(&self, 
		addr: I2CAddr,
		transac_confired: impl FnOnce(
			&Self::Transaction
		),
	) -> bool {
		(*self).safestart(addr, transac_confired)
	}
	
	#[inline(always)]
	unsafe fn start(
		&self,
		addr: I2CAddr
	) -> (bool, Self::Transaction) {
		(*self).start(addr)
	}
}


pub trait I2CGenTransaction {
	fn write(&self, data: u8) -> bool;
	unsafe fn stop(&self);
}

/*impl<'a, T> I2CGenMaster for &'a T where T: I2CGenMaster {
	#[inline(always)]
	fn scan(&self, success: impl FnMut(I2CAddr), noexists: impl FnMut(I2CAddr)) {
		T::scan(&self, success, noexists)
	}

type WriteTransaction;

fn start<const ADDR: I2CAddr>(&self) -> Self::WriteTransaction {
        todo!()
    }
}
*/
