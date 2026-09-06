//! APBC clock ownership and supplied UART source frequencies.

use crate::apbc::{TwsiClockReset, UartClockReset, UartClockSource};
use core::marker::PhantomData;
pub use embedded_time::rate::Hertz;
use volatile_register::{RW, WO};

/// UART source frequencies supplied by the platform, not measured by the HAL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clocks {
    pll1_m3d128: Option<Hertz>,
    slow_uart1: Option<Hertz>,
    slow_uart2: Option<Hertz>,
}

impl Clocks {
    /// Records unknown source frequencies without assuming BootROM defaults.
    pub const fn unknown() -> Self {
        Self {
            pll1_m3d128: None,
            slow_uart1: None,
            slow_uart2: None,
        }
    }

    /// Records nonzero UART source frequencies supplied by the platform.
    pub fn new(
        pll1_m3d128: Option<Hertz>,
        slow_uart1: Option<Hertz>,
        slow_uart2: Option<Hertz>,
    ) -> Result<Self, Error> {
        if [pll1_m3d128, slow_uart1, slow_uart2]
            .into_iter()
            .flatten()
            .any(|frequency| frequency.0 == 0)
        {
            return Err(Error::ZeroFrequency);
        }
        Ok(Self {
            pll1_m3d128,
            slow_uart1,
            slow_uart2,
        })
    }

    /// Returns the supplied frequency for a UART functional-clock source.
    pub const fn uart_source(&self, source: UartClockSource) -> Option<Hertz> {
        match source {
            UartClockSource::Pll1M3D128 => self.pll1_m3d128,
            UartClockSource::SlowUart1 => self.slow_uart1,
            UartClockSource::SlowUart2 => self.slow_uart2,
        }
    }
}

/// An invalid UART clock configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// A supplied running clock frequency is zero.
    ZeroFrequency,
    /// A UART bus or functional gate is disabled, or reset is asserted.
    Disabled,
    /// The UART source selector has a reserved encoding.
    ReservedSource,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::ZeroFrequency => "zero UART source frequency",
            Self::Disabled => "UART clock disabled or reset asserted",
            Self::ReservedSource => "reserved UART clock source",
        })
    }
}

impl core::error::Error for Error {}

/// A supported SoC-specific APBC UART identity.
///
/// # Safety
/// Each implementor must identify exactly one physical UART in one SoC;
/// different UART instances must not share the same identity type.
pub unsafe trait UartId {}

/// An exclusive UART clock token borrowed by its matching driver.
#[must_use]
pub struct UartClock<'a, I: UartId> {
    inner: UartClockRef<'a>,
    _identity: PhantomData<fn() -> I>,
}

impl<'a, I: UartId> UartClock<'a, I> {
    /// Acquires a UART clock register without reading or writing hardware.
    ///
    /// # Safety
    /// Exclusively own UART I's clock register for 'a, without recreating owners;
    /// its mapping and upstream power/clocks must stay valid, enabling UART access
    /// when the gate is enabled; no external writer may change these conditions,
    /// and supplied frequencies must remain correct, including after drop or forget.
    pub const unsafe fn from_register(register: &'a RW<UartClockReset>, clocks: Clocks) -> Self {
        Self {
            inner: UartClockRef {
                register,
                clocks,
                _not_send_sync: PhantomData,
            },
            _identity: PhantomData,
        }
    }

    /// Returns the supplied input frequency, or None when it remains unknown.
    pub fn frequency(&self) -> Option<Hertz> {
        self.inner.frequency()
    }

    /// Records platform-verified source frequencies without modifying hardware.
    ///
    /// # Safety
    /// Supplied frequencies must match the hardware and remain correct while
    /// this token or a driver uses them; this method does not configure clocks.
    pub unsafe fn set_frequencies(&mut self, clocks: Clocks) {
        self.inner.clocks = clocks;
    }

    pub(crate) fn borrow(&mut self) -> &UartClockRef<'a> {
        &self.inner
    }
}

// This capability never exposes the APBC register or a public untyped constructor.
// No Drop implementation disables gates or recreates the original APBC owner.
pub(crate) struct UartClockRef<'a> {
    register: &'a RW<UartClockReset>,
    clocks: Clocks,
    _not_send_sync: PhantomData<*mut ()>,
}

impl UartClockRef<'_> {
    pub(crate) fn frequency(&self) -> Option<Hertz> {
        self.register
            .read()
            .clock_source()
            .and_then(|source| self.clocks.uart_source(source))
    }

    pub(crate) fn check(&self) -> Result<(), Error> {
        let value = self.register.read();
        if !value.is_enabled() {
            return Err(Error::Disabled);
        }
        value.clock_source().ok_or(Error::ReservedSource)?;
        Ok(())
    }
}

/// A supported SoC-specific APBC I²C identity.
///
/// # Safety
/// Each implementor must identify exactly one physical I²C controller in one
/// SoC; different controllers must not share the same identity type.
pub unsafe trait I2cId {}

/// An exclusive I²C clock token preserving its register's readback restrictions.
#[must_use]
pub struct I2cClock<'a, I: I2cId> {
    register: I2cClockRegister<'a>,
    _identity: PhantomData<fn() -> I>,
    _not_send_sync: PhantomData<*mut ()>,
}

enum I2cClockRegister<'a> {
    ReadWrite(&'a RW<TwsiClockReset>),
    WriteOnly(&'a WO<TwsiClockReset>),
}

impl<'a, I: I2cId> I2cClock<'a, I> {
    /// Acquires a readable I²C clock register without accessing hardware.
    ///
    /// # Safety
    /// The register must control I²C I and stay mapped and accessible for 'a;
    /// exclusive access must transfer to this token without recreating owners.
    pub const unsafe fn from_register(register: &'a RW<TwsiClockReset>) -> Self {
        Self {
            register: I2cClockRegister::ReadWrite(register),
            _identity: PhantomData,
            _not_send_sync: PhantomData,
        }
    }

    /// Acquires a write-only I²C clock register without accessing hardware.
    ///
    /// # Safety
    /// The register must control I²C I and stay mapped and accessible for 'a;
    /// exclusive access must transfer to this token without recreating owners.
    pub const unsafe fn from_write_only_register(register: &'a WO<TwsiClockReset>) -> Self {
        Self {
            register: I2cClockRegister::WriteOnly(register),
            _identity: PhantomData,
            _not_send_sync: PhantomData,
        }
    }

    /// Reads the clock state, or returns None when hardware readback is unusable.
    pub fn readback(&self) -> Option<TwsiClockReset> {
        match self.register {
            I2cClockRegister::ReadWrite(register) => Some(register.read()),
            I2cClockRegister::WriteOnly(_register) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum TestUart0 {}
    enum TestUart10 {}
    // SAFETY: These identities name distinct simulated UARTs within each fixture.
    unsafe impl UartId for TestUart0 {}
    // SAFETY: This identity never names TestUart0's clock register.
    unsafe impl UartId for TestUart10 {}

    #[test]
    fn supplied_frequencies_are_optional_and_nonzero() {
        let clocks = Clocks::new(Some(Hertz(57_600_000)), None, Some(Hertz(48_000_000))).unwrap();
        assert_eq!(
            clocks.uart_source(UartClockSource::Pll1M3D128),
            Some(Hertz(57_600_000))
        );
        assert_eq!(clocks.uart_source(UartClockSource::SlowUart1), None);
        assert_eq!(
            clocks.uart_source(UartClockSource::SlowUart2),
            Some(Hertz(48_000_000))
        );
        assert_eq!(
            Clocks::new(None, Some(Hertz(0)), None),
            Err(Error::ZeroFrequency)
        );
    }

    #[test]
    fn adoption_does_not_write_or_assume_a_running_gate() {
        // SAFETY: The initialized RAM fixture contains only integer MMIO wrappers.
        let registers: crate::apbc::k1::RegisterBlock = unsafe { core::mem::zeroed() };
        // SAFETY: This test exclusively owns the simulated APBC registers.
        let clock = unsafe {
            UartClock::<TestUart0>::from_register(&registers.uart0_clock_reset, Clocks::unknown())
        };
        assert_eq!(registers.uart0_clock_reset.read().bits(), 0);
        assert_eq!(clock.inner.check(), Err(Error::Disabled));
        assert_eq!(clock.frequency(), None);
    }

    #[test]
    fn each_k3_token_uses_its_own_register() {
        // SAFETY: Every register field is an integer MMIO wrapper.
        let registers: crate::apbc::k3::RegisterBlock = unsafe { core::mem::zeroed() };
        // SAFETY: Exclusive writes to this initialized RAM fixture.
        unsafe {
            registers
                .uart0_clock_reset
                .write(UartClockReset::from_bits(3));
            registers
                .uart10_clock_reset
                .write(UartClockReset::from_bits(0x23));
        }
        let frequencies =
            Clocks::new(Some(Hertz(57_600_000)), None, Some(Hertz(48_000_000))).unwrap();
        // SAFETY: This test owns all clock registers and has no concurrent writers.
        let mut clock0 = unsafe {
            UartClock::<TestUart0>::from_register(&registers.uart0_clock_reset, frequencies)
        };
        // SAFETY: TestUart10 owns a disjoint simulated register.
        let clock10 = unsafe {
            UartClock::<TestUart10>::from_register(&registers.uart10_clock_reset, frequencies)
        };
        assert_eq!(clock0.inner.check(), Ok(()));
        assert_eq!(clock10.inner.check(), Ok(()));
        assert_eq!(clock10.frequency(), Some(Hertz(48_000_000)));
        // SAFETY: Update only the supplied metadata for this simulated register.
        unsafe { clock0.set_frequencies(Clocks::unknown()) };
        assert_eq!(clock0.frequency(), None);
        // SAFETY: Deliberately emulate a disabled clock in the RAM fixture.
        unsafe {
            registers
                .uart0_clock_reset
                .write(UartClockReset::from_bits(4))
        };
        assert_eq!(clock0.inner.check(), Err(Error::Disabled));
        // SAFETY: Deliberately inject a reserved selector in simulated RAM.
        unsafe {
            registers
                .uart10_clock_reset
                .write(UartClockReset::from_bits(0x73))
        };
        assert_eq!(clock10.inner.check(), Err(Error::ReservedSource));
    }

    #[test]
    fn i2c_tokens_preserve_readback_restrictions() {
        enum TestI2c8 {}
        // SAFETY: This identity names the sole simulated I²C controller below.
        unsafe impl I2cId for TestI2c8 {}
        // SAFETY: Initialized integer-cell fixtures, never physical MMIO.
        let readable: RW<TwsiClockReset> = unsafe { core::mem::zeroed() };
        // SAFETY: Exclusive initialization of RAM.
        unsafe { readable.write(TwsiClockReset::from_bits(3)) };
        // SAFETY: This fixture exclusively represents the simulated I²C clock.
        let clock = unsafe { I2cClock::<TestI2c8>::from_register(&readable) };
        assert!(clock.readback().unwrap().is_enabled());
        // SAFETY: A separate initialized integer-cell fixture.
        let write_only: WO<TwsiClockReset> = unsafe { core::mem::zeroed() };
        // SAFETY: The second fixture represents a separate write-only simulation.
        let clock = unsafe { I2cClock::<TestI2c8>::from_write_only_register(&write_only) };
        assert_eq!(clock.readback(), None);
    }
}
