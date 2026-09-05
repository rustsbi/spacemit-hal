//! GPIO register access types and chip-specific layouts.

use core::cell::UnsafeCell;

/// A 32-bit register whose status bits are cleared by writing one.
///
/// Unlike a general read-write register, this type intentionally provides no
/// read-modify-write operation: writing back a value read from the register
/// would acknowledge every pending bit in that value.
#[repr(transparent)]
pub struct ReadWriteOneToClear(UnsafeCell<u32>);

impl ReadWriteOneToClear {
    /// Reads the currently asserted status bits.
    #[inline(always)]
    pub fn read(&self) -> u32 {
        // SAFETY: `self` denotes an aligned 32-bit MMIO register and this is a
        // single volatile read with the width required by the register map.
        unsafe { self.0.get().read_volatile() }
    }

    /// Clears exactly the status bits selected by `mask`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that acknowledging every selected pending event
    /// is valid for the current peripheral and interrupt state.
    #[inline(always)]
    pub unsafe fn clear(&self, mask: u32) {
        // SAFETY: upheld by the caller; this performs one exact volatile write
        // and never reads or modifies unrelated status bits.
        unsafe { self.0.get().write_volatile(mask) }
    }
}

/// K1 and M1 GPIO register layout.
pub mod k1;

/// K3 GPIO register layout.
pub mod k3;

#[cfg(test)]
mod tests {
    use super::ReadWriteOneToClear;
    use core::mem::{align_of, size_of};

    #[test]
    fn rw1c_register_layout() {
        assert_eq!(size_of::<ReadWriteOneToClear>(), 4);
        assert_eq!(align_of::<ReadWriteOneToClear>(), 4);
    }
}
