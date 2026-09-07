use super::{CounterClock, CounterId, Error, Hertz};
use crate::{apbc::UartClockSource, apbs, mpmu};
use core::num::NonZeroU32;

/// A K1/M1 shared-clock configurator that leaves unspecified gates unchanged.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Strict {
    reference: Option<NonZeroU32>,
    i2c: Option<bool>,
    uart: Option<bool>,
    qspi: Option<bool>,
}

impl Strict {
    /// Creates a configuration without hardware access.
    #[inline]
    pub const fn new() -> Self {
        Self {
            reference: None,
            i2c: None,
            uart: None,
            qspi: None,
        }
    }

    /// Specifies the board's physical PLL reference frequency.
    #[inline]
    pub const fn reference_clock(mut self, frequency: Hertz) -> Self {
        self.reference = NonZeroU32::new(frequency.0);
        self
    }

    /// Selects the shared PLL1 / 78 I²C source gate state.
    #[inline]
    pub const fn i2c_clock(mut self, enabled: bool) -> Self {
        self.i2c = Some(enabled);
        self
    }

    /// Selects the shared PLL1 * 3 / 128 UART source gate state.
    #[inline]
    pub const fn uart_clock(mut self, enabled: bool) -> Self {
        self.uart = Some(enabled);
        self
    }

    /// Selects the shared PLL1 / 23 QSPI source gate state.
    #[inline]
    pub const fn qspi_clock(mut self, enabled: bool) -> Self {
        self.qspi = Some(enabled);
        self
    }

    /// Validates live PLL1 settings and applies shared gates without retuning PLL1.
    /// Requires the counter to use the fixed reference source.
    #[inline]
    pub fn configure<'a>(
        self,
        apbs: impl apbs::Instance<'a, RegisterBlock = apbs::k1::RegisterBlock>,
        mpmu: impl mpmu::Instance<'a, RegisterBlock = mpmu::k1::RegisterBlock>,
        counter: &'a mut CounterClock<impl CounterId>,
    ) -> Result<Clocks<'a>, Error> {
        let apbs = apbs.register_block();
        let mpmu = mpmu.register_block();
        let pll1 = self.read_pll(apbs, mpmu)?;
        let reference = self.reference.ok_or(Error::UnsupportedPll)?;
        let counter = counter.frequency(Hertz(reference.get()))?;
        let (parent, gates) = self.apply_gates(apbs, mpmu)?;
        Ok(Clocks {
            _borrow: core::marker::PhantomData,
            pll1,
            counter,
            i2c_enabled: parent.is_d4_enabled() && gates.is_pll1_d78_enabled(),
            qspi_enabled: parent.is_d23_enabled(),
            uart: uart_sources(
                pll1,
                parent.bits(),
                gates.bits(),
                mpmu.slow_uart1_clock.read(),
                mpmu.slow_uart2_clock.read(),
            ),
        })
    }

    #[inline]
    fn read_pll(
        self,
        apbs: &apbs::k1::RegisterBlock,
        mpmu: &mpmu::k1::RegisterBlock,
    ) -> Result<Hertz, Error> {
        self.validate_pll(
            apbs.pll1_software_control1.read(),
            apbs.pll1_software_control3.read(),
            mpmu.pll_status.read().is_pll1_locked(),
        )
    }

    #[inline]
    fn apply_gates(
        self,
        apbs: &apbs::k1::RegisterBlock,
        mpmu: &mpmu::k1::RegisterBlock,
    ) -> Result<(apbs::k1::Pll1SoftwareControl2, mpmu::ApplicationClockGate), Error> {
        let (parent, gates) = self.gates(
            apbs.pll1_software_control2.read(),
            mpmu.application_clock_gate.read(),
        );
        if self.i2c.is_some() || self.uart.is_some() || self.qspi.is_some() {
            // SAFETY: configure retains both exclusive Instances for this sequence.
            unsafe {
                apbs.pll1_software_control2.write(parent);
                super::io_fence();
                mpmu.application_clock_gate.write(gates);
                super::io_fence();
            }
            if apbs.pll1_software_control2.read() != parent
                || mpmu.application_clock_gate.read() != gates
            {
                return Err(Error::Readback);
            }
        }
        Ok((parent, gates))
    }

    #[inline]
    fn validate_pll(self, control1: u32, control3: u32, locked: bool) -> Result<Hertz, Error> {
        // Linux ccu-k1.c rate table and ccu_pll.c matching; never infer the reset state.
        if self.reference != NonZeroU32::new(24_000_000)
            || control1 != 0x0050_dd64
            || control3 & 0x7fff_ffff != 0x330c_cccd
        {
            return Err(Error::UnsupportedPll);
        }
        // BootROM can leave SWCR3[31] clear while POSR reports PLL1 locked.
        if !locked {
            return Err(Error::PllUnlocked);
        }
        Ok(Hertz(2_457_600_000))
    }

    #[inline]
    fn gates(
        self,
        mut parent: apbs::k1::Pll1SoftwareControl2,
        mut gates: mpmu::ApplicationClockGate,
    ) -> (apbs::k1::Pll1SoftwareControl2, mpmu::ApplicationClockGate) {
        if let Some(enabled) = self.i2c {
            if enabled {
                parent = parent.with_d4_enabled(true);
            }
            gates = gates.with_pll1_d78_enabled(enabled);
        }
        if let Some(enabled) = self.uart {
            if enabled {
                parent = parent.with_d8_enabled(true);
            }
            gates = gates.with_pll1_m3d128_enabled(enabled);
        }
        if let Some(enabled) = self.qspi {
            parent = parent.with_d23_enabled(enabled);
        }
        (parent, gates)
    }
}

/// Validated K1/M1 clocks retaining exclusive clock-controller borrows.
// Bindings may outlive this immutable snapshot; it must not reconfigure clocks.
pub struct Clocks<'a> {
    _borrow: core::marker::PhantomData<(&'a mut (), *mut ())>,
    pll1: Hertz,
    counter: Hertz,
    i2c_enabled: bool,
    qspi_enabled: bool,
    uart: [Option<NonZeroU32>; 3],
}

impl Clocks<'_> {
    /// Returns the enabled PLL1 / 23 QSPI source frequency in whole hertz.
    #[inline]
    pub const fn qspi_source(&self) -> Option<Hertz> {
        if self.qspi_enabled {
            Some(Hertz(self.pll1.0 / 23))
        } else {
            None
        }
    }

    /// Returns the enabled PLL1/78 frequency, rounded down to whole hertz.
    #[inline]
    pub const fn i2c_source(&self) -> Option<Hertz> {
        if self.i2c_enabled {
            Some(Hertz(self.pll1.0 / 78))
        } else {
            None
        }
    }

    /// Returns the counter frequency validated against its source selection.
    #[inline]
    pub const fn counter(&self) -> Hertz {
        self.counter
    }

    /// Returns the enabled UART source frequency rounded down to whole hertz.
    #[inline]
    pub const fn uart_source(&self, source: UartClockSource) -> Option<Hertz> {
        match self.uart_frequency(source) {
            Some(frequency) => Some(Hertz(frequency.get())),
            None => None,
        }
    }

    #[inline(always)]
    pub(super) const fn uart_frequency(&self, source: UartClockSource) -> Option<NonZeroU32> {
        self.uart[source as usize]
    }

    /// Returns the PLL1 frequency decoded from hardware configuration.
    #[inline]
    pub const fn pll1(&self) -> Hertz {
        self.pll1
    }
}

#[inline]
fn uart_sources(
    pll1: Hertz,
    parent: u32,
    gates: u32,
    slow1: u32,
    slow2: u32,
) -> [Option<NonZeroU32>; 3] {
    // Linux ccu-k1.c and ccu_ddn.c: Fout = Fin * denominator / (2 * numerator).
    #[inline]
    fn slow(parent: u32, value: u32) -> Option<NonZeroU32> {
        let numerator = u64::from((value >> 16) & 0x1fff);
        let denominator = u64::from(value & 0x1fff);
        let divisor = 2 * numerator;
        if divisor == 0 || denominator == 0 {
            return None;
        }
        let rate = u64::from(parent) * denominator;
        NonZeroU32::new(u32::try_from(rate / divisor).ok()?)
    }
    let d8 = parent & (1 << 7) != 0;
    let d4 = parent & (1 << 3) != 0;
    [
        if d8 && gates & (1 << 8) != 0 {
            NonZeroU32::new(pll1.0 / 128 * 3)
        } else {
            None
        },
        if d8 { slow(pll1.0 / 16, slow1) } else { None },
        if d4 && gates & (1 << 15) != 0 {
            slow(pll1.0 / 4, slow2)
        } else {
            None
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apbc::k1::CounterClockControl,
        counter::{self, CounterDelay},
    };
    use volatile_register::RW;

    #[test]
    fn frequency_snapshots_use_nonzero_niches() {
        assert_eq!(core::mem::size_of::<Strict>(), 8);
        assert_eq!(core::mem::size_of::<Clocks<'_>>(), 24);
    }

    macro_rules! counter_fixture {
        () => {{
            enum TestCounter {}
            // SAFETY: Zero-valid RAM, local to the containing test.
            static mut REGISTER: RW<CounterClockControl> = unsafe { core::mem::zeroed() };
            // SAFETY: This identity only names this test's permanently allocated selector.
            unsafe impl CounterId for TestCounter {
                const CLOCK_REGISTER: *const RW<CounterClockControl> =
                    core::ptr::addr_of!(REGISTER);
            }
            // SAFETY: This test owns its separate fixture and does not share it.
            unsafe {
                (
                    CounterClock::<TestCounter>::__new(),
                    &*TestCounter::CLOCK_REGISTER,
                )
            }
        }};
    }

    impl Clocks<'_> {
        pub(crate) const fn for_test(i2c_enabled: bool) -> Self {
            Self {
                _borrow: core::marker::PhantomData,
                pll1: Hertz(2_457_600_000),
                counter: Hertz(24_000_000),
                i2c_enabled,
                qspi_enabled: false,
                uart: [None; 3],
            }
        }
    }

    struct Block<T>(T);

    // SAFETY: Each borrow exclusively owns stable simulated APBS registers.
    unsafe impl<'a> apbs::Instance<'a> for &'a mut Block<apbs::k1::RegisterBlock> {
        type RegisterBlock = apbs::k1::RegisterBlock;

        fn register_block(self) -> &'a Self::RegisterBlock {
            &self.0
        }
    }

    // SAFETY: Each borrow exclusively owns stable simulated MPMU registers.
    unsafe impl<'a> mpmu::Instance<'a> for &'a mut Block<mpmu::k1::RegisterBlock> {
        type RegisterBlock = mpmu::k1::RegisterBlock;

        fn register_block(self) -> &'a Self::RegisterBlock {
            &self.0
        }
    }

    // SAFETY: Exclusive test memory remains valid and unchanged for the borrow.
    unsafe impl<'a> counter::Instance<'a> for &'a mut Block<counter::k1::RegisterBlock> {
        fn register_block(self) -> &'a counter::k1::RegisterBlock {
            &self.0
        }
    }

    #[test]
    fn counter_clock_is_accepted_by_safe_delay_constructor() {
        // SAFETY: All fixtures contain zero-valid integer register wrappers.
        let (mut apbs, mut mpmu, mut counter): (
            Block<apbs::k1::RegisterBlock>,
            Block<mpmu::k1::RegisterBlock>,
            Block<counter::k1::RegisterBlock>,
        ) = unsafe { core::mem::zeroed() };
        let (mut clock, selector) = counter_fixture!();
        // SAFETY: Initialize private RAM, including the otherwise read-only PLL status.
        unsafe {
            apbs.0.pll1_software_control1.write(0x0050_dd64);
            apbs.0.pll1_software_control3.write(0x330c_cccd);
            core::ptr::addr_of!(mpmu.0.pll_status)
                .cast_mut()
                .cast::<u32>()
                .write(1 << 27);
            selector.write(CounterClockControl::from_bits(0x02dc_0000));
        }
        let clocks = Strict::new()
            .reference_clock(Hertz(24_000_000))
            .i2c_clock(true)
            .configure(&mut apbs, &mut mpmu, &mut clock)
            .unwrap();
        assert_eq!(clocks.counter(), Hertz(24_000_000u32));
        assert_eq!(clocks.i2c_source(), Some(Hertz(31_507_692u32)));
        let result: Result<CounterDelay<'_>, counter::Error> =
            CounterDelay::new(&mut counter, &clocks);
        assert!(matches!(result, Err(counter::Error::NotAdvancing)));
        assert!(counter.0.control.read().is_enabled());
        assert_eq!(selector.read().bits(), 0x02dc_0000);
        assert_eq!(counter.0.frequency_id.read(), 0);
    }

    #[test]
    fn i2c_readiness_requires_both_shared_gates() {
        let (mut counter, _selector) = counter_fixture!();
        for parent in [0, 1 << 3] {
            for gate in [0, 1 << 6] {
                // SAFETY: Zero-valid register fixtures in private RAM.
                let (mut apbs, mut mpmu): (
                    Block<apbs::k1::RegisterBlock>,
                    Block<mpmu::k1::RegisterBlock>,
                ) = unsafe { core::mem::zeroed() };
                // SAFETY: Initialize a fixed-reference, locked PLL fixture.
                unsafe {
                    apbs.0.pll1_software_control1.write(0x0050_dd64);
                    apbs.0.pll1_software_control3.write(0xb30c_cccd);
                    apbs.0
                        .pll1_software_control2
                        .write(apbs::k1::Pll1SoftwareControl2::from_bits(parent));
                    mpmu.0
                        .application_clock_gate
                        .write(mpmu::ApplicationClockGate::from_bits(gate));
                    core::ptr::addr_of!(mpmu.0.pll_status)
                        .cast_mut()
                        .cast::<u32>()
                        .write(1 << 27);
                }
                let clocks = Strict::new()
                    .reference_clock(Hertz(24_000_000))
                    .configure(&mut apbs, &mut mpmu, &mut counter)
                    .unwrap();
                assert_eq!(
                    clocks.i2c_source(),
                    (parent != 0 && gate != 0).then_some(Hertz(31_507_692u32)),
                );
                drop(clocks);
                let clocks = Strict::new()
                    .reference_clock(Hertz(24_000_000))
                    .i2c_clock(false)
                    .configure(&mut apbs, &mut mpmu, &mut counter)
                    .unwrap();
                assert_eq!(clocks.i2c_source(), None);
            }
        }
    }

    const CONFIG: Strict = Strict::new()
        .reference_clock(Hertz(24_000_000))
        .i2c_clock(true)
        .uart_clock(true);

    #[test]
    fn uart_binding_checks_its_static_register_without_writing() {
        use crate::{
            apbc::UartClockReset,
            clock::{UartClock, UartId},
        };
        enum TestUart {}
        // SAFETY: Zero-valid RAM used exclusively by this test.
        static mut REGISTER: RW<UartClockReset> = unsafe { core::mem::zeroed() };
        // SAFETY: This identity names only this test's permanent selector.
        unsafe impl UartId for TestUart {
            const CLOCK_REGISTER: *const RW<UartClockReset> = core::ptr::addr_of!(REGISTER);
        }
        // SAFETY: No other thread or test accesses this fixture.
        let register = unsafe { &*TestUart::CLOCK_REGISTER };
        let mut clock = unsafe { UartClock::<TestUart>::__new() };
        let mut clocks = Clocks::for_test(false);
        clocks.uart[0] = NonZeroU32::new(57_600_000);
        for (bits, expected) in [
            (0, Err(Error::Disabled)),
            (3, Ok(Hertz(57_600_000))),
            (0x13, Err(Error::UnknownFrequency)),
            (0x33, Err(Error::ReservedSource)),
        ] {
            // SAFETY: Exclusive RAM initialization before acquiring the binding.
            unsafe { register.write(UartClockReset::from_bits(bits)) };
            assert_eq!(
                clock.with_clock(&clocks).map(|clock| clock.frequency()),
                expected
            );
            assert_eq!(register.read().bits(), bits);
        }
    }

    #[test]
    fn live_pll_is_checked() {
        for control3 in [0x330c_cccd, 0xb30c_cccd] {
            assert_eq!(
                CONFIG.validate_pll(0x0050_dd64, control3, true),
                Ok(Hertz(2_457_600_000))
            );
            assert_eq!(
                CONFIG.validate_pll(0x0050_dd64, control3, false),
                Err(Error::PllUnlocked)
            );
        }
        assert_eq!(
            CONFIG.validate_pll(0, 0xb30c_cccd, true),
            Err(Error::UnsupportedPll)
        );
        assert_eq!(
            Strict::new().validate_pll(0x0050_dd64, 0xb30c_cccd, true),
            Err(Error::UnsupportedPll)
        );
    }

    #[test]
    fn gates_preserve_unspecified_state() {
        for bits in [0, u32::MAX, 0xaaaa_aaaa] {
            let parent = apbs::k1::Pll1SoftwareControl2::from_bits(bits);
            let gates = mpmu::ApplicationClockGate::from_bits(bits);
            assert_eq!(Strict::new().gates(parent, gates), (parent, gates));
            let (enabled, outputs) = CONFIG.gates(parent, gates);
            assert_eq!(enabled.bits(), bits | (1 << 3) | (1 << 7));
            assert_eq!(outputs.bits(), bits | (1 << 6) | (1 << 8));
            let (disabled, outputs) = CONFIG
                .i2c_clock(false)
                .uart_clock(false)
                .gates(parent, gates);
            assert_eq!(disabled, parent);
            assert_eq!(outputs.bits(), bits & !((1 << 6) | (1 << 8)));
            let (enabled, outputs) = Strict::new().qspi_clock(true).gates(parent, gates);
            assert_eq!(enabled.bits(), bits | (1 << 20));
            assert_eq!(outputs, gates);
            let (disabled, outputs) = Strict::new().qspi_clock(false).gates(parent, gates);
            assert_eq!(disabled.bits(), bits & !(1 << 20));
            assert_eq!(outputs, gates);
        }
    }

    #[test]
    fn uart_frequencies_follow_live_paths() {
        let pll = Hertz(2_457_600_000);
        let parent = (1 << 7) | (1 << 3);
        let gates = (1 << 8) | (1 << 15);
        let slow1 = (125 << 16) | 24;
        let slow2 = (32 << 16) | 5;
        assert_eq!(
            uart_sources(pll, parent, gates, slow1, slow2).map(|rate| rate.map(NonZeroU32::get)),
            [Some(57_600_000), Some(14_745_600), Some(48_000_000)]
        );
        assert_eq!(uart_sources(pll, 0, gates, slow1, slow2), [None; 3]);
        assert_eq!(uart_sources(pll, parent, 0, 0, 0), [None; 3]);
        // MUSE Card M1 BootROM's fractional-Hz source remains usable.
        assert_eq!(
            uart_sources(pll, 0x58b9_8fff, 0x0024_f113, 0x1fbd_0600, 0x1800_03c0)
                .map(|rate| rate.map(NonZeroU32::get)),
            [Some(57_600_000), Some(14_518_744), Some(48_000_000)]
        );
    }
}
