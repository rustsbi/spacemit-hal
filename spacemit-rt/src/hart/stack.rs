use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
};

use super::Error;

/// A sixteen-byte-aligned stack containing N bytes.
#[repr(C, align(16))]
pub struct Stack<const N: usize> {
    memory: UnsafeCell<MaybeUninit<[u8; N]>>,
    taken: AtomicBool,
}

// SAFETY: take grants the only allocation; the memory is never otherwise exposed.
unsafe impl<const N: usize> Sync for Stack<N> {}

impl<const N: usize> Stack<N> {
    /// Creates an unallocated stack.
    #[inline]
    pub const fn new() -> Self {
        Self {
            memory: UnsafeCell::new(MaybeUninit::uninit()),
            taken: AtomicBool::new(false),
        }
    }

    /// Takes this stack after checking space and alignment for F and startup.
    #[inline]
    pub fn take<F>(&'static self) -> Result<StackAllocation<F>, Error> {
        let base = self.memory.get().cast::<u8>();
        let top = base as usize + N;
        let address = top
            .checked_sub(core::mem::size_of::<F>().max(1))
            .ok_or(Error::StackTooSmall)?
            & !(core::mem::align_of::<F>() - 1);
        let sp = address & !15;
        if sp < base as usize || sp - (base as usize) < 128 {
            return Err(Error::StackTooSmall);
        }
        if self.taken.swap(true, Ordering::AcqRel) {
            return Err(Error::WouldBlock);
        }
        Ok(StackAllocation {
            sp,
            // SAFETY: The aligned F slot is in this allocation, including for ZSTs.
            argument: unsafe { base.add(address - base as usize).cast() },
        })
    }
}

impl<const N: usize> Default for Stack<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// A permanently owned stack allocation checked for F.
pub struct StackAllocation<F> {
    sp: usize,
    argument: *mut F,
}

impl<F> StackAllocation<F> {
    #[inline]
    pub(super) fn prepare(self, entry: F) -> (usize, *mut ()) {
        // SAFETY: take reserved this unique, aligned slot for the same F.
        unsafe { self.argument.write(entry) };
        (self.sp, self.argument.cast())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_is_taken_once_and_closure_is_aligned() {
        static STACK: Stack<1024> = Stack::new();
        #[repr(align(64))]
        struct Capture([u8; 64]);
        let capture = Capture([7; 64]);
        let (sp, argument) = STACK.take().unwrap().prepare(capture);
        assert_eq!(sp % 16, 0);
        assert_eq!(argument as usize % 64, 0);
        assert!(matches!(STACK.take::<Capture>(), Err(Error::WouldBlock)));
        // SAFETY: this test owns the allocation and never starts a hart on it.
        assert_eq!(unsafe { argument.cast::<Capture>().read() }.0, [7; 64]);
    }

    #[test]
    fn insufficient_space_does_not_take_the_stack() {
        static EMPTY: Stack<0> = Stack::new();
        static SMALL: Stack<128> = Stack::new();
        static STACK: Stack<144> = Stack::new();
        assert!(matches!(EMPTY.take::<()>(), Err(Error::StackTooSmall)));
        assert!(matches!(SMALL.take::<()>(), Err(Error::StackTooSmall)));
        assert!(matches!(
            STACK.take::<[u8; 17]>(),
            Err(Error::StackTooSmall)
        ));
        let (sp, argument) = STACK.take().unwrap().prepare([7u8; 16]);
        assert_eq!(sp - STACK.memory.get() as usize, 128);
        // SAFETY: No hart uses this stack, and prepare initialized the array.
        assert_eq!(unsafe { argument.cast::<[u8; 16]>().read() }, [7; 16]);
    }
}
