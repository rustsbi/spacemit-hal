use core::{marker::PhantomData, ptr::NonNull};
use volatile_register::{RO, WO};

/// A borrowed GPIO bank register view.
#[derive(Clone, Copy)]
pub(in crate::gpio) struct BankRegisters<'a> {
    base: NonNull<u8>,
    stride: u8,
    _borrow: PhantomData<&'a RO<u32>>,
}

impl<'a> BankRegisters<'a> {
    // The caller retains a complete live bank for 'a, with 4- or 12-byte spacing.
    #[inline(always)]
    pub(super) const unsafe fn new(base: NonNull<u8>, stride: u8) -> Self {
        Self {
            base,
            stride,
            _borrow: PhantomData,
        }
    }

    #[inline(always)]
    unsafe fn register<R>(&self, index: usize) -> &R {
        // SAFETY: The accessor selects the matching type and index in this live bank.
        unsafe {
            &*self
                .base
                .as_ptr()
                .add(index * self.stride as usize)
                .cast::<R>()
        }
    }

    #[inline(always)]
    pub(in crate::gpio) fn pin_level(&self) -> &RO<u32> {
        // SAFETY: PLR is the first, read-only register.
        unsafe { self.register(0) }
    }

    #[inline(always)]
    pub(in crate::gpio) fn pin_output_set(&self) -> &WO<u32> {
        // SAFETY: PSR is the third register, a write-only set alias.
        unsafe { self.register(2) }
    }

    #[inline(always)]
    pub(in crate::gpio) fn pin_output_clear(&self) -> &WO<u32> {
        // SAFETY: PCR is the fourth register, a write-only clear alias.
        unsafe { self.register(3) }
    }

    #[inline(always)]
    pub(in crate::gpio) fn direction_set(&self) -> &WO<u32> {
        // SAFETY: SDR is the eighth register, a write-only set alias.
        unsafe { self.register(7) }
    }

    #[inline(always)]
    pub(in crate::gpio) fn direction_clear(&self) -> &WO<u32> {
        // SAFETY: CDR is the ninth register, a write-only clear alias.
        unsafe { self.register(8) }
    }
}

const _: () =
    assert!(core::mem::size_of::<BankRegisters<'_>>() == 2 * core::mem::size_of::<usize>());
