use super::{Error, Hertz};
use crate::apbc::k1::CounterClockControl;
use core::marker::PhantomData;
use volatile_register::RW;

/// A K1/M1 generic-counter clock-source identity.
///
/// # Safety
/// Identify one physical counter and its permanently mapped APBC selector.
pub unsafe trait CounterId {
    /// The matching counter clock-source register.
    const CLOCK_REGISTER: *const RW<CounterClockControl>;
}

/// Exclusive K1/M1 counter clock-source token.
pub struct CounterClock<I: CounterId> {
    _identity: PhantomData<fn() -> I>,
    _not_send_sync: PhantomData<*mut ()>,
}

impl<I: CounterId> CounterClock<I> {
    /// Wraps the counter clock-source register without accessing it.
    ///
    /// # Safety
    /// Exclusively own I's permanently mapped and accessible APBC selector;
    /// no other owner may change its source while borrowed by a consumer.
    #[doc(hidden)]
    #[inline(always)]
    pub const unsafe fn __new() -> Self {
        Self {
            _identity: PhantomData,
            _not_send_sync: PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) fn frequency(&self, reference: Hertz) -> Result<Hertz, Error> {
        // SAFETY: The token owns this permanently mapped clock-source register.
        if !unsafe { &*I::CLOCK_REGISTER }
            .read()
            .is_reference_selected()
        {
            return Err(Error::UnsupportedCounterClock);
        }
        Ok(reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_fixed_reference_mode_without_rewriting_it() {
        enum TestCounter {}
        // SAFETY: Zero-valid RAM, only used by this test.
        static mut REGISTER: RW<CounterClockControl> = unsafe { core::mem::zeroed() };
        // SAFETY: This private identity names only this test's permanent fixture.
        unsafe impl CounterId for TestCounter {
            const CLOCK_REGISTER: *const RW<CounterClockControl> = core::ptr::addr_of!(REGISTER);
        }
        // SAFETY: No other test or thread uses this fixture.
        let register = unsafe { &*TestCounter::CLOCK_REGISTER };
        for upper in [0, 0x02dc_0000, 0xffff_fffc] {
            for mode in 0..4 {
                let bits = upper | mode;
                // SAFETY: Exclusive initialization before acquiring the token.
                unsafe { register.write(CounterClockControl::from_bits(bits)) };
                // SAFETY: A single simulated clock register remains live here.
                let clock = unsafe { CounterClock::<TestCounter>::__new() };
                let expected = if mode == 0 {
                    Ok(Hertz(24_000_000))
                } else {
                    Err(Error::UnsupportedCounterClock)
                };
                assert_eq!(clock.frequency(Hertz(24_000_000)), expected);
                assert_eq!(register.read().bits(), bits);
            }
        }
    }
}
