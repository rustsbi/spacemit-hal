//! Shared APBC register values.

// UART/TWSI clock gates and mux: drivers/clk/spacemit/ccu-k1.c and ccu-k3.c.
// UART/TWSI reset: drivers/reset/spacemit/reset-spacemit-k1.c and reset-spacemit-k3.c.
// https://github.com/torvalds/linux/tree/master/drivers/clk/spacemit
// https://github.com/torvalds/linux/tree/master/drivers/reset/spacemit

/// UART functional-clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UartClockSource {
    /// PLL1-derived `pll1_m3d128` clock.
    Pll1M3D128 = 0,
    /// First slow-UART clock generator.
    SlowUart1 = 1,
    /// Second slow-UART clock generator.
    SlowUart2 = 2,
}

/// UART clock gates, source selection, and active-high reset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct UartClockReset(u32);

impl UartClockReset {
    const BUS_CLOCK_ENABLE: u32 = 1 << 0;
    const FUNCTIONAL_CLOCK_ENABLE: u32 = 1 << 1;
    const RESET: u32 = 1 << 2;
    const CLOCK_SOURCE_SHIFT: u32 = 4;
    const CLOCK_SOURCE_MASK: u32 = 0x7 << Self::CLOCK_SOURCE_SHIFT;

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

    /// Returns whether the APB bus clock gate is enabled.
    #[inline]
    pub const fn is_bus_clock_enabled(self) -> bool {
        self.0 & Self::BUS_CLOCK_ENABLE != 0
    }

    /// Sets the APB bus clock gate in this value.
    #[inline]
    pub const fn with_bus_clock_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::BUS_CLOCK_ENABLE) | enabled as u32)
    }

    /// Returns whether the UART functional clock gate is enabled.
    #[inline]
    pub const fn is_functional_clock_enabled(self) -> bool {
        self.0 & Self::FUNCTIONAL_CLOCK_ENABLE != 0
    }

    /// Sets the UART functional clock gate in this value.
    #[inline]
    pub const fn with_functional_clock_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::FUNCTIONAL_CLOCK_ENABLE) | ((enabled as u32) << 1))
    }

    /// Returns whether UART reset is asserted.
    #[inline]
    pub const fn is_reset_asserted(self) -> bool {
        self.0 & Self::RESET != 0
    }

    /// Sets the active-high UART reset in this value.
    #[inline]
    pub const fn with_reset_asserted(self, asserted: bool) -> Self {
        Self((self.0 & !Self::RESET) | ((asserted as u32) << 2))
    }

    /// Returns whether both clock gates are enabled and reset is released.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        self.is_bus_clock_enabled()
            && self.is_functional_clock_enabled()
            && !self.is_reset_asserted()
    }

    /// Returns the selected clock source, or `None` for a reserved selector.
    #[inline]
    pub const fn clock_source(self) -> Option<UartClockSource> {
        match (self.0 & Self::CLOCK_SOURCE_MASK) >> Self::CLOCK_SOURCE_SHIFT {
            0 => Some(UartClockSource::Pll1M3D128),
            1 => Some(UartClockSource::SlowUart1),
            2 => Some(UartClockSource::SlowUart2),
            _ => None,
        }
    }

    /// Selects the UART functional-clock source in this value.
    #[inline]
    pub const fn with_clock_source(self, source: UartClockSource) -> Self {
        Self((self.0 & !Self::CLOCK_SOURCE_MASK) | ((source as u32) << Self::CLOCK_SOURCE_SHIFT))
    }
}

/// TWSI (I²C) functional-clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TwsiClockSource {
    /// PLL1 divided by 78.
    Pll1D78 = 0,
    /// PLL1 divided by 48.
    Pll1D48 = 1,
    /// PLL1 divided by 40.
    Pll1D40 = 2,
}

/// TWSI (I²C) clock gates, source selection, and active-high reset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct TwsiClockReset(u32);

impl TwsiClockReset {
    const BUS_CLOCK_ENABLE: u32 = 1 << 0;
    const FUNCTIONAL_CLOCK_ENABLE: u32 = 1 << 1;
    const RESET: u32 = 1 << 2;
    const CLOCK_SOURCE_SHIFT: u32 = 4;
    const CLOCK_SOURCE_MASK: u32 = 0x7 << Self::CLOCK_SOURCE_SHIFT;

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

    /// Returns whether the APB bus clock gate is enabled in this value.
    #[inline]
    pub const fn is_bus_clock_enabled(self) -> bool {
        self.0 & Self::BUS_CLOCK_ENABLE != 0
    }

    /// Sets the APB bus clock gate in this value.
    #[inline]
    pub const fn with_bus_clock_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::BUS_CLOCK_ENABLE) | enabled as u32)
    }

    /// Returns whether the TWSI functional clock gate is enabled in this value.
    #[inline]
    pub const fn is_functional_clock_enabled(self) -> bool {
        self.0 & Self::FUNCTIONAL_CLOCK_ENABLE != 0
    }

    /// Sets the TWSI functional clock gate in this value.
    #[inline]
    pub const fn with_functional_clock_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::FUNCTIONAL_CLOCK_ENABLE) | ((enabled as u32) << 1))
    }

    /// Returns whether TWSI reset is asserted in this value.
    #[inline]
    pub const fn is_reset_asserted(self) -> bool {
        self.0 & Self::RESET != 0
    }

    /// Sets the active-high TWSI reset in this value.
    #[inline]
    pub const fn with_reset_asserted(self, asserted: bool) -> Self {
        Self((self.0 & !Self::RESET) | ((asserted as u32) << 2))
    }

    /// Returns whether this value enables both clock gates and releases reset.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        self.is_bus_clock_enabled()
            && self.is_functional_clock_enabled()
            && !self.is_reset_asserted()
    }

    /// Returns the selected clock source, or `None` for a reserved selector.
    #[inline]
    pub const fn clock_source(self) -> Option<TwsiClockSource> {
        match (self.0 & Self::CLOCK_SOURCE_MASK) >> Self::CLOCK_SOURCE_SHIFT {
            0 => Some(TwsiClockSource::Pll1D78),
            1 => Some(TwsiClockSource::Pll1D48),
            2 => Some(TwsiClockSource::Pll1D40),
            _ => None,
        }
    }

    /// Selects the TWSI functional-clock source in this value.
    #[inline]
    pub const fn with_clock_source(self, source: TwsiClockSource) -> Self {
        Self((self.0 & !Self::CLOCK_SOURCE_MASK) | ((source as u32) << Self::CLOCK_SOURCE_SHIFT))
    }
}

#[cfg(test)]
mod tests {
    use super::{TwsiClockReset, TwsiClockSource, UartClockReset, UartClockSource};
    use core::mem::{align_of, size_of};
    use volatile_register::{RW, WO};

    #[test]
    fn uart_clock_reset_layout() {
        assert_eq!(size_of::<UartClockReset>(), 4);
        assert_eq!(align_of::<UartClockReset>(), 4);
        assert_eq!(size_of::<RW<UartClockReset>>(), 4);
        assert_eq!(align_of::<RW<UartClockReset>>(), 4);
    }

    #[test]
    fn gates_and_reset_match_bootrom_console_check() {
        for high_bits in [0, 0x10, 0x20, 0xffff_fff8] {
            for state in 0..8 {
                let bits = high_bits | state;
                let value = UartClockReset::from_bits(bits);
                assert_eq!(value.bits(), bits);
                assert_eq!(value.is_bus_clock_enabled(), state & 1 != 0);
                assert_eq!(value.is_functional_clock_enabled(), state & 2 != 0);
                assert_eq!(value.is_reset_asserted(), state & 4 != 0);
                assert_eq!(value.is_enabled(), state == 3);
            }
        }
    }

    #[test]
    fn gate_and_reset_updates_preserve_other_bits() {
        for bits in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
            let value = UartClockReset::from_bits(bits);
            for enabled in [false, true] {
                let bus = value.with_bus_clock_enabled(enabled);
                assert_eq!(bus.bits() & !1, bits & !1);
                assert_eq!(bus.is_bus_clock_enabled(), enabled);

                let functional = value.with_functional_clock_enabled(enabled);
                assert_eq!(functional.bits() & !2, bits & !2);
                assert_eq!(functional.is_functional_clock_enabled(), enabled);

                let reset = value.with_reset_asserted(enabled);
                assert_eq!(reset.bits() & !4, bits & !4);
                assert_eq!(reset.is_reset_asserted(), enabled);
            }
        }
    }

    #[test]
    fn clock_source_encodings_and_reserved_selectors() {
        for (source, encoding) in [
            (UartClockSource::Pll1M3D128, 0),
            (UartClockSource::SlowUart1, 1),
            (UartClockSource::SlowUart2, 2),
        ] {
            for bits in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
                let value = UartClockReset::from_bits(bits).with_clock_source(source);
                assert_eq!(value.clock_source(), Some(source));
                assert_eq!(value.bits() & 0x70, encoding << 4);
                assert_eq!(value.bits() & !0x70, bits & !0x70);
            }
        }
        for selector in 3..8 {
            assert_eq!(
                UartClockReset::from_bits(selector << 4).clock_source(),
                None
            );
        }
    }

    #[test]
    fn twsi_clock_reset_layout() {
        assert_eq!(size_of::<TwsiClockReset>(), 4);
        assert_eq!(align_of::<TwsiClockReset>(), 4);
        assert_eq!(size_of::<RW<TwsiClockReset>>(), 4);
        assert_eq!(align_of::<RW<TwsiClockReset>>(), 4);
        assert_eq!(size_of::<WO<TwsiClockReset>>(), 4);
        assert_eq!(align_of::<WO<TwsiClockReset>>(), 4);
    }

    #[test]
    fn twsi_gate_and_reset_bits() {
        for high_bits in [0, 0x10, 0x20, 0xffff_fff8] {
            for state in 0..8 {
                let value = TwsiClockReset::from_bits(high_bits | state);
                assert_eq!(value.bits(), high_bits | state);
                assert_eq!(value.is_bus_clock_enabled(), state & 1 != 0);
                assert_eq!(value.is_functional_clock_enabled(), state & 2 != 0);
                assert_eq!(value.is_reset_asserted(), state & 4 != 0);
                assert_eq!(value.is_enabled(), state == 3);
            }
        }
    }

    #[test]
    fn twsi_gate_and_reset_updates_preserve_other_bits() {
        for bits in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
            let value = TwsiClockReset::from_bits(bits);
            for enabled in [false, true] {
                let bus = value.with_bus_clock_enabled(enabled);
                assert_eq!(bus.bits() & !1, bits & !1);
                assert_eq!(bus.is_bus_clock_enabled(), enabled);

                let functional = value.with_functional_clock_enabled(enabled);
                assert_eq!(functional.bits() & !2, bits & !2);
                assert_eq!(functional.is_functional_clock_enabled(), enabled);

                let reset = value.with_reset_asserted(enabled);
                assert_eq!(reset.bits() & !4, bits & !4);
                assert_eq!(reset.is_reset_asserted(), enabled);
            }
        }
    }

    #[test]
    fn twsi_clock_source_encodings_and_reserved_selectors() {
        for (source, encoding) in [
            (TwsiClockSource::Pll1D78, 0),
            (TwsiClockSource::Pll1D48, 1),
            (TwsiClockSource::Pll1D40, 2),
        ] {
            for bits in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
                let value = TwsiClockReset::from_bits(bits).with_clock_source(source);
                assert_eq!(value.clock_source(), Some(source));
                assert_eq!(value.bits() & 0x70, encoding << 4);
                assert_eq!(value.bits() & !0x70, bits & !0x70);
            }
        }
        for selector in 3..8 {
            assert_eq!(
                TwsiClockReset::from_bits(selector << 4).clock_source(),
                None
            );
        }
    }

    #[test]
    fn twsi_cold_boot_values_match_vendor_sequence() {
        let reset = TwsiClockReset::from_bits(0)
            .with_clock_source(TwsiClockSource::Pll1D78)
            .with_reset_asserted(true);
        let clocked = reset
            .with_bus_clock_enabled(true)
            .with_functional_clock_enabled(true);
        let running = clocked.with_reset_asserted(false);
        assert_eq!([reset.bits(), clocked.bits(), running.bits()], [4, 7, 3]);
        assert!(running.is_enabled());
    }
}
