use super::{Clocks, Error, Hertz};
use crate::apbc::TwsiClockReset;
use core::marker::PhantomData;
use volatile_register::WO;

/// A supported SoC-specific APBC I²C identity.
///
/// # Safety
/// Each implementor must identify exactly one physical I²C controller in one
/// SoC; different controllers must not share the same identity type.
pub unsafe trait I2cId {
    /// Write-only view of the matching, permanently mapped APBC clock register.
    const CLOCK_REGISTER: *const WO<TwsiClockReset>;
}

/// An exclusive I²C clock token with a statically selected register.
#[must_use]
pub struct I2cClock<I: I2cId> {
    _identity: PhantomData<fn() -> I>,
    _not_send_sync: PhantomData<*mut ()>,
}

impl<I: I2cId> I2cClock<I> {
    /// Acquires an I²C clock token without accessing hardware.
    ///
    /// # Safety
    /// I's clock register must stay permanently mapped and accessible;
    /// exclusive access must transfer to this token without recreating owners.
    /// Upstream power and register access must remain valid;
    /// no external writer may change its clock/reset while borrowed by a driver.
    #[doc(hidden)]
    #[inline(always)]
    pub const unsafe fn __new() -> Self {
        Self {
            _identity: PhantomData,
            _not_send_sync: PhantomData,
        }
    }

    /// Binds the verified PLL1/78 source without accessing the APBC register.
    #[inline(always)]
    pub fn with_clock<'b>(&'b mut self, clocks: &Clocks<'b>) -> Result<I2cFrequency<'b, I>, Error> {
        let frequency = clocks.i2c_source().ok_or(Error::Disabled)?;
        Ok(I2cFrequency {
            inner: I2cFrequencyRef {
                _source: PhantomData,
                frequency,
            },
            _identity: PhantomData,
        })
    }
}

/// A PLL1/78 input frequency retaining its I²C gate and shared-source borrows.
#[must_use]
pub struct I2cFrequency<'a, I: I2cId> {
    pub(crate) inner: I2cFrequencyRef<'a>,
    _identity: PhantomData<fn() -> I>,
}

impl<I: I2cId> I2cFrequency<'_, I> {
    /// Returns the verified input frequency, rounded down to whole hertz.
    #[inline]
    pub const fn frequency(&self) -> Hertz {
        self.inner.frequency
    }

    #[inline]
    pub(crate) fn enable(&mut self, delay: &mut impl embedded_hal::delay::DelayNs) {
        // Vendor cold-boot sequence; never read K1 TWSI8's write-only register.
        for bits in [4, 7, 3] {
            let value = TwsiClockReset::from_bits(bits);
            // SAFETY: The binding retains exclusive register access and its verified source.
            unsafe {
                (*I::CLOCK_REGISTER).write(value);
            }
            super::io_fence();
            delay.delay_us(100);
        }
    }
}

pub(crate) struct I2cFrequencyRef<'a> {
    _source: PhantomData<(&'a mut (), Clocks<'a>)>,
    frequency: Hertz,
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    // SAFETY: Initialized RAM, accessed only while LOCK is held.
    static mut REGISTER: WO<TwsiClockReset> = unsafe { core::mem::zeroed() };

    enum TestI2c8 {}
    // SAFETY: This identity names the private simulated I²C controller.
    unsafe impl I2cId for TestI2c8 {
        const CLOCK_REGISTER: *const WO<TwsiClockReset> = core::ptr::addr_of!(REGISTER);
    }

    fn read_fixture() -> u32 {
        // SAFETY: All callers hold LOCK; inspect RAM, never write-only MMIO.
        unsafe { TestI2c8::CLOCK_REGISTER.cast::<u32>().read_volatile() }
    }

    #[test]
    fn i2c_tokens_preserve_register_access_modes() {
        assert_eq!(core::mem::size_of::<I2cClock<TestI2c8>>(), 0);
        let _: *const WO<TwsiClockReset> = TestI2c8::CLOCK_REGISTER;
    }

    #[test]
    fn binding_checks_shared_source_without_writing_apbc() {
        let _guard = LOCK.lock().unwrap();
        // SAFETY: LOCK provides exclusive access to the permanent test fixture.
        unsafe { (*TestI2c8::CLOCK_REGISTER).write(TwsiClockReset::from_bits(0xa5)) };
        let mut clock = unsafe { I2cClock::<TestI2c8>::__new() };
        let disabled = Clocks::for_test(false);
        assert!(matches!(clock.with_clock(&disabled), Err(Error::Disabled)));
        let clocks = Clocks::for_test(true);
        let frequency = clock.with_clock(&clocks).unwrap();
        assert_eq!(frequency.frequency(), Hertz(31_507_692u32));
        assert_eq!(Some(frequency.frequency()), clocks.i2c_source());
        assert_eq!(read_fixture(), 0xa5);
    }

    #[test]
    fn bound_frequency_preserves_cold_boot_sequence() {
        let _guard = LOCK.lock().unwrap();
        struct Delay {
            values: [u32; 3],
            count: usize,
        }
        impl embedded_hal::delay::DelayNs for Delay {
            fn delay_ns(&mut self, ns: u32) {
                assert_eq!(ns, 100_000);
                self.values[self.count] = read_fixture();
                self.count += 1;
            }
        }
        // SAFETY: LOCK provides exclusive access to the permanent test fixture.
        let mut clock = unsafe { I2cClock::<TestI2c8>::__new() };
        let clocks = Clocks::for_test(true);
        let mut frequency = clock.with_clock(&clocks).unwrap();
        let mut delay = Delay {
            values: [0; 3],
            count: 0,
        };
        frequency.enable(&mut delay);
        assert_eq!(delay.values, [4, 7, 3]);
        assert_eq!(delay.count, 3);
    }
}
