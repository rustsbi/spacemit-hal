//! Shared APMU register values.

// QSPI gates, divider and mux: drivers/clk/spacemit/ccu-k1.c and ccu-k3.c.
// Active-low reset bits: drivers/reset/spacemit/reset-spacemit-k1.c and -k3.c.
// https://github.com/torvalds/linux/blob/master/drivers/clk/spacemit/ccu-k1.c
// https://github.com/torvalds/linux/blob/master/drivers/clk/spacemit/ccu-k3.c
// https://github.com/torvalds/linux/blob/master/drivers/clk/spacemit/ccu_mix.c
// Source selectors are SoC-specific: K1 uses PLL1 / 23 at selector 5, whereas
// K3 uses the configurable pll1_dx source and reserves selectors 4 and 7.
// Bit 12 is a self-clearing frequency-change request, not ordinary RW state;
// do not replay it in RMW operations while a frequency change is pending.

/// QSPI clock gates, source selector, divider, and active-low resets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct QspiClockReset(u32);

impl QspiClockReset {
    const BUS_RESET_RELEASE: u32 = 1 << 0;
    const RESET_RELEASE: u32 = 1 << 1;
    const BUS_CLOCK_ENABLE: u32 = 1 << 3;
    const FUNCTIONAL_CLOCK_ENABLE: u32 = 1 << 4;
    const CLOCK_SOURCE_SHIFT: u32 = 6;
    const CLOCK_SOURCE_MASK: u32 = 7 << Self::CLOCK_SOURCE_SHIFT;
    const CLOCK_DIVIDER_SHIFT: u32 = 9;
    const CLOCK_DIVIDER_MASK: u32 = 7 << Self::CLOCK_DIVIDER_SHIFT;
    const FREQUENCY_CHANGE: u32 = 1 << 12;

    /// Creates a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Returns the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Returns whether QSPI bus reset is asserted.
    #[inline]
    pub const fn is_bus_reset_asserted(self) -> bool {
        self.0 & Self::BUS_RESET_RELEASE == 0
    }

    /// Sets the active-low QSPI bus reset in this value.
    #[inline]
    pub const fn with_bus_reset_asserted(self, asserted: bool) -> Self {
        Self((self.0 & !Self::BUS_RESET_RELEASE) | (!asserted as u32))
    }

    /// Returns whether QSPI functional reset is asserted.
    #[inline]
    pub const fn is_reset_asserted(self) -> bool {
        self.0 & Self::RESET_RELEASE == 0
    }

    /// Sets the active-low QSPI functional reset in this value.
    #[inline]
    pub const fn with_reset_asserted(self, asserted: bool) -> Self {
        Self((self.0 & !Self::RESET_RELEASE) | ((!asserted as u32) << 1))
    }

    /// Returns whether the QSPI bus clock gate is enabled.
    #[inline]
    pub const fn is_bus_clock_enabled(self) -> bool {
        self.0 & Self::BUS_CLOCK_ENABLE != 0
    }

    /// Sets the QSPI bus clock gate in this value.
    #[inline]
    pub const fn with_bus_clock_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::BUS_CLOCK_ENABLE) | ((enabled as u32) << 3))
    }

    /// Returns whether the QSPI functional clock gate is enabled.
    #[inline]
    pub const fn is_functional_clock_enabled(self) -> bool {
        self.0 & Self::FUNCTIONAL_CLOCK_ENABLE != 0
    }

    /// Sets the QSPI functional clock gate in this value.
    #[inline]
    pub const fn with_functional_clock_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::FUNCTIONAL_CLOCK_ENABLE) | ((enabled as u32) << 4))
    }

    /// Returns the raw SoC-specific clock source selector.
    #[inline]
    pub const fn clock_source_selector(self) -> u8 {
        ((self.0 & Self::CLOCK_SOURCE_MASK) >> Self::CLOCK_SOURCE_SHIFT) as u8
    }

    /// Sets the raw SoC-specific selector, panicking outside `0..=7`.
    #[inline]
    pub const fn with_clock_source_selector(self, selector: u8) -> Self {
        assert!(selector < 8, "QSPI clock source selector must be in 0..=7");
        Self((self.0 & !Self::CLOCK_SOURCE_MASK) | ((selector as u32) << Self::CLOCK_SOURCE_SHIFT))
    }

    /// Returns the functional-clock divisor in `1..=8`.
    #[inline]
    pub const fn clock_divider(self) -> u8 {
        (((self.0 & Self::CLOCK_DIVIDER_MASK) >> Self::CLOCK_DIVIDER_SHIFT) + 1) as u8
    }

    /// Sets the functional-clock divisor, panicking outside `1..=8`.
    #[inline]
    pub const fn with_clock_divider(self, divider: u8) -> Self {
        assert!(
            divider >= 1 && divider <= 8,
            "QSPI clock divider must be in 1..=8"
        );
        Self(
            (self.0 & !Self::CLOCK_DIVIDER_MASK)
                | (((divider - 1) as u32) << Self::CLOCK_DIVIDER_SHIFT),
        )
    }

    /// Returns whether a self-clearing frequency-change request is pending.
    #[inline]
    pub const fn is_frequency_change_pending(self) -> bool {
        self.0 & Self::FREQUENCY_CHANGE != 0
    }
}

#[cfg(test)]
mod tests {
    use super::QspiClockReset;
    use core::mem::{align_of, size_of};
    use volatile_register::RW;

    #[test]
    fn qspi_clock_reset_layout() {
        assert_eq!(size_of::<QspiClockReset>(), 4);
        assert_eq!(align_of::<QspiClockReset>(), 4);
        assert_eq!(size_of::<RW<QspiClockReset>>(), 4);
        assert_eq!(align_of::<RW<QspiClockReset>>(), 4);
    }

    #[test]
    fn gates_active_low_resets_and_frequency_change() {
        for high_bits in [0, 0x1000, 0xffff_ffe0] {
            for state in 0..32 {
                let value = QspiClockReset::from_bits(high_bits | state);
                assert_eq!(value.bits(), high_bits | state);
                assert_eq!(value.is_bus_reset_asserted(), state & 1 == 0);
                assert_eq!(value.is_reset_asserted(), state & 2 == 0);
                assert_eq!(value.is_bus_clock_enabled(), state & 8 != 0);
                assert_eq!(value.is_functional_clock_enabled(), state & 16 != 0);
                assert_eq!(value.is_frequency_change_pending(), high_bits & 0x1000 != 0);
            }
        }
    }

    #[test]
    fn gate_and_reset_updates_preserve_other_bits() {
        for bits in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
            let value = QspiClockReset::from_bits(bits);
            for enabled in [false, true] {
                let bus = value.with_bus_clock_enabled(enabled);
                assert_eq!(bus.bits() & !8, bits & !8);
                assert_eq!(bus.is_bus_clock_enabled(), enabled);
                let functional = value.with_functional_clock_enabled(enabled);
                assert_eq!(functional.bits() & !16, bits & !16);
                assert_eq!(functional.is_functional_clock_enabled(), enabled);
                let bus_reset = value.with_bus_reset_asserted(enabled);
                assert_eq!(bus_reset.bits() & !1, bits & !1);
                assert_eq!(bus_reset.is_bus_reset_asserted(), enabled);
                let reset = value.with_reset_asserted(enabled);
                assert_eq!(reset.bits() & !2, bits & !2);
                assert_eq!(reset.is_reset_asserted(), enabled);
            }
        }
    }

    #[test]
    fn source_selector_and_divider_encodings() {
        for bits in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
            for selector in 0..8 {
                let value = QspiClockReset::from_bits(bits).with_clock_source_selector(selector);
                assert_eq!(value.clock_source_selector(), selector);
                assert_eq!(value.bits() & (7 << 6), u32::from(selector) << 6);
                assert_eq!(value.bits() & !(7 << 6), bits & !(7 << 6));
            }
            for divider in 1..=8 {
                let value = QspiClockReset::from_bits(bits).with_clock_divider(divider);
                assert_eq!(value.clock_divider(), divider);
                assert_eq!(value.bits() & (7 << 9), u32::from(divider - 1) << 9);
                assert_eq!(value.bits() & !(7 << 9), bits & !(7 << 9));
            }
        }
    }

    #[test]
    #[should_panic(expected = "QSPI clock source selector must be in 0..=7")]
    fn rejects_invalid_source_selector() {
        QspiClockReset::from_bits(0).with_clock_source_selector(8);
    }

    #[test]
    #[should_panic(expected = "QSPI clock divider must be in 1..=8")]
    fn rejects_zero_divider() {
        QspiClockReset::from_bits(0).with_clock_divider(0);
    }

    #[test]
    #[should_panic(expected = "QSPI clock divider must be in 1..=8")]
    fn rejects_oversized_divider() {
        QspiClockReset::from_bits(0).with_clock_divider(9);
    }

    #[test]
    fn k1_cold_boot_configuration_matches_raw_masks() {
        for bits in [0, 0xffff_efff, 0xaaaa_aaaa & !0x1000] {
            let reset = QspiClockReset::from_bits(bits)
                .with_clock_source_selector(5)
                .with_clock_divider(5)
                .with_bus_clock_enabled(false)
                .with_functional_clock_enabled(false)
                .with_bus_reset_asserted(true)
                .with_reset_asserted(true);
            let expected = (bits & !((7 << 6) | (7 << 9) | 0x1b)) | (5 << 6) | (4 << 9);
            assert_eq!(reset.bits(), expected);
            let clocked = reset
                .with_bus_clock_enabled(true)
                .with_functional_clock_enabled(true);
            assert_eq!(clocked.bits(), expected | 0x18);
            let running = clocked
                .with_bus_reset_asserted(false)
                .with_reset_asserted(false);
            assert_eq!(running.bits(), expected | 0x1b);
            assert!(!running.is_frequency_change_pending());
        }
    }
}
