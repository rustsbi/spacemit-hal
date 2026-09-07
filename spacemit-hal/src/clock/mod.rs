//! Clock configuration and frequency-bound peripheral tokens.

mod counter;
mod i2c;
mod strict;
mod uart;
pub use counter::{CounterClock, CounterId};
pub use i2c::{I2cClock, I2cFrequency, I2cId};
pub use strict::{Clocks, Strict};
pub use uart::{UartClock, UartFrequency, UartId};

pub use embedded_time::rate::Hertz;
pub(crate) use i2c::I2cFrequencyRef;
pub(crate) use uart::UartFrequencyRef;

/// An invalid clock configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// PLL1 is not locked.
    PllUnlocked,
    /// The reference frequency or PLL1 register configuration is unsupported.
    UnsupportedPll,
    /// The counter uses low-frequency stepping or automatic source switching.
    UnsupportedCounterClock,
    /// A clock gate did not retain its requested setting.
    Readback,
    /// A supplied running clock frequency is zero.
    ZeroFrequency,
    /// A required clock gate is disabled, or reset is asserted.
    Disabled,
    /// The UART source selector has a reserved encoding.
    ReservedSource,
    /// The selected UART source has no verified frequency.
    UnknownFrequency,
    /// The requested baud rate cannot use a 16-bit divisor within 3% error.
    ImpossibleBaudrate,
}

impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::PllUnlocked => "PLL1 is not locked",
            Self::UnsupportedPll => "unsupported PLL1 configuration or reference frequency",
            Self::UnsupportedCounterClock => "counter requires a fixed reference clock",
            Self::Readback => "clock gate readback mismatch",
            Self::ZeroFrequency => "zero UART source frequency",
            Self::Disabled => "clock disabled or reset asserted",
            Self::ReservedSource => "reserved UART clock source",
            Self::UnknownFrequency => "unverified UART source frequency",
            Self::ImpossibleBaudrate => "impossible baudrate",
        })
    }
}

impl core::error::Error for Error {}

#[inline]
pub(crate) fn io_fence() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    riscv::asm::fence();
    // Host tests access private RAM rather than device memory.
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
