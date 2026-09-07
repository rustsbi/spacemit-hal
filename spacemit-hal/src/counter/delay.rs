use super::{Instance, k1};
use crate::clock::{Clocks, Hertz};
use core::marker::PhantomData;
use embedded_hal::delay::DelayNs;

/// Failure to initialize a generic-counter delay provider.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// A zero tick frequency cannot describe a running counter.
    ZeroFrequency,
    /// No change in the counter was observed within the startup polling budget.
    NotAdvancing,
}

/// Blocking K1/M1 delays using the MMIO counter and a validated clock.
/// Delays round up with a one-tick phase guard; a stopped counter blocks forever.
/// Drop and [`free`](Self::free) leave the shared timebase running.
pub struct CounterDelay<'a> {
    counter: &'a k1::RegisterBlock,
    frequency: Hertz,
    _clocks: PhantomData<Clocks<'a>>,
}

impl<'a> CounterDelay<'a> {
    /// Enables the counter using [`Clocks::counter`] and checks up to 100,000 samples.
    /// A failed check leaves the counter enabled; the input token is consumed.
    #[inline]
    pub fn new(counter: impl Instance<'a>, clocks: &Clocks<'a>) -> Result<Self, Error> {
        // SAFETY: Instance transfers valid, exclusive counter access; Clocks
        // supplies its verified frequency and retains the source-controller borrows.
        unsafe {
            Self::initialize(
                counter.register_block(),
                clocks.counter(),
                k1::RegisterBlock::value,
            )
        }
    }

    // The private reader allows tests to model a running or stalled counter.
    // Requires Instance's access guarantees and a stable verified frequency.
    #[inline]
    unsafe fn initialize(
        counter: &'a k1::RegisterBlock,
        frequency: Hertz,
        mut read: impl FnMut(&k1::RegisterBlock) -> u64,
    ) -> Result<Self, Error> {
        if frequency.0 == 0 {
            return Err(Error::ZeroFrequency);
        }
        let control = &counter.control;
        // SAFETY: The caller guarantees valid MMIO and exclusive configuration
        // access. CNTCR.EN is RW; preserve HDBG and all other bits, as vendor SPL
        // timer_init does. Do not write the counter value or frequency registers.
        unsafe { control.write(control.read().with_enabled(true)) };
        super::io_fence();
        let start = read(counter);
        let mut remaining = 100_000;
        while remaining != 0 {
            remaining -= 1;
            if read(counter) != start {
                return Ok(Self {
                    counter,
                    frequency,
                    _clocks: PhantomData,
                });
            }
        }
        Err(Error::NotAdvancing)
    }
    /// Returns the configured tick rate (not a measured frequency).
    #[inline]
    pub const fn frequency(&self) -> Hertz {
        self.frequency
    }

    /// Returns the register reference, leaving the counter running.
    #[inline]
    pub fn free(self) -> &'a k1::RegisterBlock {
        self.counter
    }
}

impl DelayNs for CounterDelay<'_> {
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        wait_ticks(ticks(ns, self.frequency.0, 1_000_000_000), || {
            self.counter.value()
        });
    }

    #[inline]
    fn delay_us(&mut self, us: u32) {
        wait_ticks(ticks(us, self.frequency.0, 1_000_000), || {
            self.counter.value()
        });
    }

    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        wait_ticks(ticks(ms, self.frequency.0, 1_000), || self.counter.value());
    }
}

// All inputs originate as u32, so the product fits in u64. With the smallest
// units_per_second (1,000), the rounded result plus its phase guard also fits.
#[inline]
fn ticks(duration: u32, frequency: u32, units_per_second: u64) -> u64 {
    if duration == 0 {
        0
    } else {
        (u64::from(duration) * u64::from(frequency)).div_ceil(units_per_second) + 1
    }
}

#[inline]
fn wait_ticks(ticks: u64, mut read: impl FnMut() -> u64) {
    if ticks == 0 {
        return;
    }
    let start = read();
    while read().wrapping_sub(start) < ticks {
        core::hint::spin_loop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counter::Control;

    struct FakeCounter(k1::RegisterBlock);

    impl FakeCounter {
        fn new(bits: u32) -> Self {
            // SAFETY: All registers accept zero in private test memory.
            let registers: k1::RegisterBlock = unsafe { core::mem::zeroed() };
            // SAFETY: Exclusive, initialized test memory, not physical MMIO.
            unsafe { registers.control.write(Control::from_bits(bits)) };
            Self(registers)
        }
    }

    #[test]
    fn enables_without_changing_other_control_bits_and_retains_reference() {
        for bits in [0, 1, 2, 3, 0xaaaa_aaaa, u32::MAX] {
            let counter = FakeCounter::new(bits);
            let mut samples = [u64::MAX, u64::MAX, 0].into_iter();
            // SAFETY: The fake owns valid test registers and a counter reader.
            let result = unsafe {
                CounterDelay::initialize(&counter.0, Hertz(24_000_000), |counter| {
                    assert!(counter.control.read().is_enabled());
                    samples.next().unwrap()
                })
            };
            let Ok(delay) = result else {
                panic!("counter should advance")
            };
            assert_eq!(delay.frequency(), Hertz(24_000_000u32));
            let registers = delay.free();
            assert!(core::ptr::eq(registers, &counter.0));
            assert_eq!(registers.control.read().bits(), bits | 1);
        }
    }

    #[test]
    fn zero_frequency_returns_plain_error_without_changing_registers() {
        let counter = FakeCounter::new(2);
        // SAFETY: The zero rate is rejected before accessing the valid fixture.
        let result = unsafe {
            CounterDelay::initialize(&counter.0, Hertz(0), |_| panic!("must not read counter"))
        };
        assert!(matches!(result, Err(Error::ZeroFrequency)));
        assert_eq!(counter.0.control.read().bits(), 2);
    }

    #[test]
    fn stalled_counter_returns_plain_error_without_disabling_it() {
        let counter = FakeCounter::new(2);
        let mut reads = 0;
        // SAFETY: The fake owns test registers; the static sample models a fault.
        let result = unsafe {
            CounterDelay::initialize(&counter.0, Hertz(24_000_000), |_| {
                reads += 1;
                42
            })
        };
        let Err(error) = result else {
            panic!("counter should stall")
        };
        assert_eq!(error, Error::NotAdvancing);
        assert_eq!(reads, 100_001);
        assert_eq!(counter.0.control.read().bits(), 3);
    }

    #[test]
    fn borrowed_counter_is_returned_still_enabled() {
        let counter = FakeCounter::new(0);
        let mut value = 0;
        // SAFETY: Exclusive test memory with a monotonically increasing reader.
        let result = unsafe {
            CounterDelay::initialize(&counter.0, Hertz(24_000_000), |_| {
                value += 1;
                value
            })
        };
        let Ok(mut delay) = result else {
            panic!("counter should advance")
        };
        // Zero delays must not wait on these non-advancing test registers.
        delay.delay_ns(0);
        delay.delay_us(0);
        delay.delay_ms(0);
        let returned = delay.free();
        assert!(returned.control.read().is_enabled());
    }

    #[test]
    fn conversion_rounds_up_and_covers_initial_tick_phase() {
        assert_eq!(ticks(0, 24_000_000, 1_000_000_000), 0);
        assert_eq!(ticks(1, 24_000_000, 1_000_000_000), 2);
        assert_eq!(ticks(41, 24_000_000, 1_000_000_000), 2);
        assert_eq!(ticks(42, 24_000_000, 1_000_000_000), 3);
        assert_eq!(ticks(125, 24_000_000, 1_000_000_000), 4);
        assert_eq!(ticks(1, 24_000_000, 1_000_000), 25);
        assert_eq!(ticks(1, 24_000_000, 1_000), 24_001);
        for units in [1_000, 1_000_000, 1_000_000_000] {
            for frequency in [1, 32_768, 24_000_000, u32::MAX] {
                for duration in [1, 41, 42, 999, 1_000, u32::MAX] {
                    let count = ticks(duration, frequency, units);
                    let requested = u128::from(duration) * u128::from(frequency);
                    let whole_ticks = u128::from(count - 1) * u128::from(units);
                    assert!(whole_ticks >= requested);
                    assert!(whole_ticks - requested < u128::from(units));
                }
            }
        }
    }

    #[test]
    fn maximum_delays_do_not_overflow() {
        for units in [1_000, 1_000_000, 1_000_000_000] {
            let expected =
                (u128::from(u32::MAX) * u128::from(u32::MAX)).div_ceil(u128::from(units)) + 1;
            assert_eq!(u128::from(ticks(u32::MAX, u32::MAX, units)), expected);
        }
    }

    #[test]
    fn waits_across_wrap_and_accepts_elapsed_ticks_during_interrupts() {
        for (count, samples) in [
            (3, [10, 10, 11, 12, 13]),
            (3, [u64::MAX - 1, u64::MAX - 1, u64::MAX, 0, 1]),
            (100, [10, 10, 11, 12, 1_000]),
        ] {
            let mut reads = samples.into_iter();
            wait_ticks(count, || {
                reads.next().expect("delay read past its deadline")
            });
            assert!(reads.next().is_none(), "delay returned early");
        }
    }
}
