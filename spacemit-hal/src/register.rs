//! Access types for registers with side effects.

use vcell::VolatileCell;

pub use crate::gpio::RW1C;

/// A read-only register whose reads acknowledge events or consume FIFO data.
#[repr(transparent)]
pub struct RC<T: Copy> {
    register: VolatileCell<T>,
}

impl<T: Copy> RC<T> {
    /// Performs a destructive volatile read.
    ///
    /// # Safety
    /// Consuming this data or acknowledging this event must be valid for the peripheral.
    #[inline(always)]
    pub unsafe fn read(&self) -> T {
        self.register.get()
    }
}

/// A read/write register with side effects and no generic modify operation.
#[repr(transparent)]
pub struct RWNoModify<T: Copy> {
    register: VolatileCell<T>,
}

impl<T: Copy> RWNoModify<T> {
    /// Performs a potentially destructive volatile read.
    ///
    /// # Safety
    /// Reading, including FIFO pops, event acknowledgements or lock acquisition, must be valid.
    #[inline(always)]
    pub unsafe fn read(&self) -> T {
        self.register.get()
    }

    /// Writes the exact value without a preceding read.
    ///
    /// # Safety
    /// The value and its hardware side effects must be valid for the current peripheral state.
    #[inline(always)]
    pub unsafe fn write(&self, value: T) {
        self.register.set(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};

    #[test]
    fn access_and_layout() {
        assert_eq!(size_of::<RWNoModify<u32>>(), 4);
        assert_eq!(align_of::<RWNoModify<u32>>(), 4);
        assert_eq!(size_of::<RC<u32>>(), 4);
        assert_eq!(align_of::<RC<u32>>(), 4);
        let register = RWNoModify {
            register: VolatileCell::new(0u32),
        };
        // SAFETY: This is owned ordinary memory, with no device side effects.
        unsafe {
            register.write(0x1234_5678);
            assert_eq!(register.read(), 0x1234_5678);
        }
    }
}
