//! Secondary-hart startup with application-owned stacks.

mod pmu;
mod stack;

pub use stack::{Stack, StackAllocation};

use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

/// A hart startup or stack allocation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// The target is the calling hart.
    InvalidHart,
    /// The stack is taken or another startup is pending or timed out.
    WouldBlock,
    /// The stack cannot hold the closure and startup frame.
    StackTooSmall,
    /// The hart did not acknowledge startup.
    Unresponsive,
}

/// Exclusive permission to start hardware hart ID once.
#[derive(Debug)]
pub struct Hart<const ID: usize> {
    _private: (),
}

static STARTING: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

#[repr(C)]
pub(crate) struct Mailbox {
    pub(crate) stack: AtomicUsize,
    pub(crate) entry: AtomicUsize,
    pub(crate) argument: AtomicPtr<()>,
}

pub(crate) static MAILBOXES: [Mailbox; 16] = [const {
    Mailbox {
        stack: AtomicUsize::new(0),
        entry: AtomicUsize::new(0),
        argument: AtomicPtr::new(core::ptr::null_mut()),
    }
}; 16];

impl<const ID: usize> Hart<ID> {
    /// Creates an exclusive hart token.
    ///
    /// # Safety
    /// ID must be valid for the selected SoC; its token must be unique, including
    /// after any previous spawn attempt.
    #[inline]
    pub(crate) const unsafe fn new() -> Self {
        Self { _private: () }
    }

    /// Returns the hardware hart ID.
    #[inline]
    pub const fn id(&self) -> usize {
        ID
    }

    /// Starts a closure, returning this token on failure and parking on return.
    ///
    /// # Safety
    ///
    /// Run an X60/X100/A100 image after boot initialization in M-mode on the
    /// selected SoC, with accessible APMU, CIU and CCI registers.
    ///
    /// The target must be powered off, with reset-invalidated caches.
    ///
    /// Reserve PMU wakeup, cluster reset vectors and CCI against other writers.
    ///
    /// The image and shared RAM must remain accessible at identical addresses
    /// with coherent mappings.
    ///
    /// The stack must be large enough for the closure's entire execution (there
    /// is no stack guard), and accessible to the target in shared, coherent RAM.
    ///
    /// Timeout does not cancel startup: the stack and closure remain transferred,
    /// and further startup requests remain blocked even with the returned token.
    #[inline]
    pub unsafe fn spawn<F>(self, stack: StackAllocation<F>, entry: F) -> Result<(), (Self, Error)>
    where
        F: FnOnce() + Send + 'static,
    {
        if ID == riscv::register::mhartid::read() {
            return Err((self, Error::InvalidHart));
        }
        // Serialize cluster-vector writes and acknowledge CCI setup before the
        // next release; a timeout leaves it locked because the hart may arrive late.
        if STARTING
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return Err((self, Error::WouldBlock));
        }
        let (sp, argument) = stack.prepare(entry);
        let mailbox = &MAILBOXES[ID];
        mailbox
            .entry
            .store(run::<F> as *const () as usize, Ordering::Relaxed);
        mailbox.argument.store(argument, Ordering::Relaxed);
        mailbox.stack.store(sp, Ordering::Release);
        // SAFETY: The caller reserved these registers, and the mailbox is complete.
        unsafe { pmu::release(ID) };
        let acknowledged = (0..1_000_000).any(|_| mailbox.stack.load(Ordering::Acquire) == 0);
        if acknowledged {
            STARTING.store(false, Ordering::Release);
            Ok(())
        } else {
            Err((self, Error::Unresponsive))
        }
    }
}

unsafe extern "C" fn run<F: FnOnce()>(argument: *mut ()) {
    // SAFETY: spawn transferred F into the allocation above the new stack top;
    // the assembly consumes its mailbox once before calling this trampoline.
    let entry = unsafe { argument.cast::<F>().read() };
    entry();
}
