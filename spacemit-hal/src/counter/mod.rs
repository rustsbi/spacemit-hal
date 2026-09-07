//! Generic counter control and blocking delays for K1/M1.
//!
//! Own or borrow `COUNTER`, using a frequency supplied by [`crate::clock::Clocks`].
//! Reads MMIO, not `time` or `mcycle`. K3's layout is not assumed to match.
//!
//! ```no_run
//! use spacemit_hal::{clock::Clocks, counter::{CounterDelay, Error, Instance}, prelude::*};
//!
//! // Pass `p.counter` for ownership, or `&mut p.counter` to borrow it.
//! fn example<'a>(counter: impl Instance<'a>, clocks: &Clocks<'a>) -> Result<(), Error> {
//!     let mut delay = CounterDelay::new(counter, clocks)?;
//!     delay.delay_us(100);
//!     delay.delay_ms(10);
//!     Ok(())
//! }
//! ```
mod delay;
mod register;
pub use delay::{CounterDelay, Error};
pub use register::{Control, k1};

/// An exclusive K1/M1 counter token or its mutable borrow.
///
/// # Safety
/// Transfer the counter controlled by K1/M1 `Clocks`, with exclusive access and
/// valid MMIO, power and upstream clocks for `'a`. Enabling it must be permitted;
/// no external writer may reset or change its value, clock or power while used.
pub unsafe trait Instance<'a> {
    /// Consumes the token or mutable borrow, retaining access for `'a`.
    fn register_block(self) -> &'a k1::RegisterBlock;
}

// Volatile access alone does not order device transactions on RISC-V.
#[inline]
fn io_fence() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    riscv::asm::fence();
    // Non-RISC-V builds exercise ordinary test memory, not K1 device memory.
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
