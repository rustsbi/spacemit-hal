//! GPIO register access types and chip-specific layouts.

use vcell::VolatileCell;

pub(super) mod bank;

/// A read/write-one-to-clear register.
#[repr(transparent)]
pub struct RW1C<T: Copy> {
    register: VolatileCell<T>,
}

impl<T: Copy> RW1C<T> {
    /// Reads status bits.
    #[inline(always)]
    pub fn read(&self) -> T {
        self.register.get()
    }

    /// Clears status bits selected by `mask`.
    ///
    /// # Safety
    /// Acknowledging every selected event must be valid for the peripheral and interrupts.
    #[inline(always)]
    pub unsafe fn clear(&self, mask: T) {
        self.register.set(mask)
    }
}

// Padding names encode byte offsets within the containing struct.
pub mod k1;
pub mod k3;

#[cfg(test)]
mod tests {
    use super::RW1C;
    use core::mem::{align_of, size_of};

    #[test]
    fn rw1c_register_layout() {
        assert_eq!(size_of::<RW1C<u8>>(), size_of::<u8>());
        assert_eq!(align_of::<RW1C<u8>>(), align_of::<u8>());
        assert_eq!(size_of::<RW1C<u16>>(), size_of::<u16>());
        assert_eq!(align_of::<RW1C<u16>>(), align_of::<u16>());
        assert_eq!(size_of::<RW1C<u32>>(), 4);
        assert_eq!(align_of::<RW1C<u32>>(), 4);
        assert_eq!(size_of::<RW1C<u64>>(), size_of::<u64>());
        assert_eq!(align_of::<RW1C<u64>>(), align_of::<u64>());
    }
}
