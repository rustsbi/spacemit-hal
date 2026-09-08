//! K1 PLL register layout.

use volatile_register::RW;

// Offsets: include/soc/spacemit/k1-syscon.h; extent: k1.dtsi.
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k1-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k1.dtsi
// Gates: https://github.com/torvalds/linux/blob/master/drivers/clk/spacemit/ccu-k1.c
// Other fields: K1 User Manual, section 9.2.4.4.2, pp. 206-207.
// Bit 0 uses Linux's K1-specific /64 meaning, not the manual's generic PLLx /1 label.

/// K1 PLL registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 64],
    /// PLL1 software control 1.
    pub pll1_software_control1: RW<u32>,
    /// PLL1 software control 2.
    pub pll1_software_control2: RW<Pll1SoftwareControl2>,
    /// PLL1 software control 3.
    pub pll1_software_control3: RW<u32>,
    _padding_0x10c: [u32; 3],
    /// PLL2 software control 1.
    pub pll2_software_control1: RW<u32>,
    /// PLL2 software control 2.
    pub pll2_software_control2: RW<u32>,
    /// PLL2 software control 3.
    pub pll2_software_control3: RW<u32>,
    /// PLL3 software control 1.
    pub pll3_software_control1: RW<u32>,
    /// PLL3 software control 2.
    pub pll3_software_control2: RW<u32>,
    /// PLL3 software control 3.
    pub pll3_software_control3: RW<u32>,
    _padding_0x130: [u32; 948],
}

/// K1 PLL1 configuration and output clock gates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Pll1SoftwareControl2(u32);

impl Pll1SoftwareControl2 {
    const D64_ENABLE: u32 = 1 << 0;
    const D2_ENABLE: u32 = 1 << 1;
    const D3_ENABLE: u32 = 1 << 2;
    const D4_ENABLE: u32 = 1 << 3;
    const D5_ENABLE: u32 = 1 << 4;
    const D6_ENABLE: u32 = 1 << 5;
    const D7_ENABLE: u32 = 1 << 6;
    const D8_ENABLE: u32 = 1 << 7;
    const ADC_ENABLE: u32 = 1 << 8;
    const DAC_ENABLE: u32 = 1 << 9;
    const D10_ENABLE: u32 = 1 << 10;
    const D100_ENABLE: u32 = 1 << 11;
    const ANALOG_TEST_ENABLE: u32 = 1 << 12;
    const CLOCK_TEST_ENABLE: u32 = 1 << 13;
    const DIGITAL_TEST_ENABLE: u32 = 1 << 14;
    const D11_ENABLE: u32 = 1 << 15;
    const D13_ENABLE: u32 = 1 << 16;
    const MONITOR_ENABLE: u32 = 1 << 19;
    const D23_ENABLE: u32 = 1 << 20;
    const DIVIDER_UPDATE_ENABLE: u32 = 1 << 21;
    const REFERENCE_BUFFER_ENABLE: u32 = 1 << 22;
    const BANDGAP_ENABLE: u32 = 1 << 23;
    const MONITOR_DIVIDER_SHIFT: u32 = 17;
    const MONITOR_DIVIDER_MASK: u32 = 0x3 << Self::MONITOR_DIVIDER_SHIFT;
    const BANDGAP_TEMPERATURE_COEFFICIENT_SHIFT: u32 = 24;
    const BANDGAP_TEMPERATURE_COEFFICIENT_MASK: u32 =
        0x3 << Self::BANDGAP_TEMPERATURE_COEFFICIENT_SHIFT;
    const BANDGAP_OUTPUT_SHIFT: u32 = 26;
    const BANDGAP_OUTPUT_MASK: u32 = 0x7 << Self::BANDGAP_OUTPUT_SHIFT;

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
    /// Returns the PLL1 / 64 output enable bit.
    #[inline]
    pub const fn is_d64_enabled(self) -> bool {
        self.0 & Self::D64_ENABLE != 0
    }
    /// Sets the PLL1 / 64 output enable bit.
    #[inline]
    pub const fn with_d64_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D64_ENABLE) | enabled as u32)
    }
    /// Returns the PLL1 / 2 output enable bit.
    #[inline]
    pub const fn is_d2_enabled(self) -> bool {
        self.0 & Self::D2_ENABLE != 0
    }
    /// Sets the PLL1 / 2 output enable bit.
    #[inline]
    pub const fn with_d2_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D2_ENABLE) | ((enabled as u32) << 1))
    }
    /// Returns the PLL1 / 3 output enable bit.
    #[inline]
    pub const fn is_d3_enabled(self) -> bool {
        self.0 & Self::D3_ENABLE != 0
    }
    /// Sets the PLL1 / 3 output enable bit.
    #[inline]
    pub const fn with_d3_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D3_ENABLE) | ((enabled as u32) << 2))
    }
    /// Returns the PLL1 / 4 output enable bit.
    #[inline]
    pub const fn is_d4_enabled(self) -> bool {
        self.0 & Self::D4_ENABLE != 0
    }
    /// Sets the PLL1 / 4 output enable bit.
    #[inline]
    pub const fn with_d4_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D4_ENABLE) | ((enabled as u32) << 3))
    }
    /// Returns the PLL1 / 5 output enable bit.
    #[inline]
    pub const fn is_d5_enabled(self) -> bool {
        self.0 & Self::D5_ENABLE != 0
    }
    /// Sets the PLL1 / 5 output enable bit.
    #[inline]
    pub const fn with_d5_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D5_ENABLE) | ((enabled as u32) << 4))
    }
    /// Returns the PLL1 / 6 output enable bit.
    #[inline]
    pub const fn is_d6_enabled(self) -> bool {
        self.0 & Self::D6_ENABLE != 0
    }
    /// Sets the PLL1 / 6 output enable bit.
    #[inline]
    pub const fn with_d6_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D6_ENABLE) | ((enabled as u32) << 5))
    }
    /// Returns the PLL1 / 7 output enable bit.
    #[inline]
    pub const fn is_d7_enabled(self) -> bool {
        self.0 & Self::D7_ENABLE != 0
    }
    /// Sets the PLL1 / 7 output enable bit.
    #[inline]
    pub const fn with_d7_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D7_ENABLE) | ((enabled as u32) << 6))
    }
    /// Returns the PLL1 / 8 output enable bit.
    #[inline]
    pub const fn is_d8_enabled(self) -> bool {
        self.0 & Self::D8_ENABLE != 0
    }
    /// Sets the PLL1 / 8 output enable bit.
    #[inline]
    pub const fn with_d8_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D8_ENABLE) | ((enabled as u32) << 7))
    }
    /// Returns the ADC clock enable bit.
    #[inline]
    pub const fn is_adc_enabled(self) -> bool {
        self.0 & Self::ADC_ENABLE != 0
    }
    /// Sets the ADC clock enable bit used when APBaux override is enabled.
    #[inline]
    pub const fn with_adc_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::ADC_ENABLE) | ((enabled as u32) << 8))
    }
    /// Returns the DAC clock enable bit.
    #[inline]
    pub const fn is_dac_enabled(self) -> bool {
        self.0 & Self::DAC_ENABLE != 0
    }
    /// Sets the DAC clock enable bit used when APBaux override is enabled.
    #[inline]
    pub const fn with_dac_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::DAC_ENABLE) | ((enabled as u32) << 9))
    }
    /// Returns the PLL1 / 10 audio output enable bit.
    #[inline]
    pub const fn is_d10_enabled(self) -> bool {
        self.0 & Self::D10_ENABLE != 0
    }
    /// Sets the PLL1 / 10 audio output enable bit.
    #[inline]
    pub const fn with_d10_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D10_ENABLE) | ((enabled as u32) << 10))
    }
    /// Returns the PLL1 / 100 audio output enable bit.
    #[inline]
    pub const fn is_d100_enabled(self) -> bool {
        self.0 & Self::D100_ENABLE != 0
    }
    /// Sets the PLL1 / 100 audio output enable bit.
    #[inline]
    pub const fn with_d100_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D100_ENABLE) | ((enabled as u32) << 11))
    }
    /// Returns the PLL1 analog test enable bit.
    #[inline]
    pub const fn is_analog_test_enabled(self) -> bool {
        self.0 & Self::ANALOG_TEST_ENABLE != 0
    }
    /// Sets the PLL1 analog test enable bit.
    #[inline]
    pub const fn with_analog_test_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::ANALOG_TEST_ENABLE) | ((enabled as u32) << 12))
    }
    /// Returns the PLL1 clock test enable bit.
    #[inline]
    pub const fn is_clock_test_enabled(self) -> bool {
        self.0 & Self::CLOCK_TEST_ENABLE != 0
    }
    /// Sets the PLL1 clock test enable bit.
    #[inline]
    pub const fn with_clock_test_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::CLOCK_TEST_ENABLE) | ((enabled as u32) << 13))
    }
    /// Returns the PLL1 digital test enable bit.
    #[inline]
    pub const fn is_digital_test_enabled(self) -> bool {
        self.0 & Self::DIGITAL_TEST_ENABLE != 0
    }
    /// Sets the PLL1 digital test enable bit.
    #[inline]
    pub const fn with_digital_test_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::DIGITAL_TEST_ENABLE) | ((enabled as u32) << 14))
    }
    /// Returns the PLL1 / 11 output enable bit.
    #[inline]
    pub const fn is_d11_enabled(self) -> bool {
        self.0 & Self::D11_ENABLE != 0
    }
    /// Sets the PLL1 / 11 output enable bit.
    #[inline]
    pub const fn with_d11_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D11_ENABLE) | ((enabled as u32) << 15))
    }
    /// Returns the PLL1 / 13 output enable bit.
    #[inline]
    pub const fn is_d13_enabled(self) -> bool {
        self.0 & Self::D13_ENABLE != 0
    }
    /// Sets the PLL1 / 13 output enable bit.
    #[inline]
    pub const fn with_d13_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D13_ENABLE) | ((enabled as u32) << 16))
    }
    /// Returns the PLL1 monitor enable bit.
    #[inline]
    pub const fn is_monitor_enabled(self) -> bool {
        self.0 & Self::MONITOR_ENABLE != 0
    }
    /// Sets the PLL1 monitor enable bit.
    #[inline]
    pub const fn with_monitor_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::MONITOR_ENABLE) | ((enabled as u32) << 19))
    }
    /// Returns the PLL1 / 23 output enable bit.
    #[inline]
    pub const fn is_d23_enabled(self) -> bool {
        self.0 & Self::D23_ENABLE != 0
    }
    /// Sets the PLL1 / 23 output enable bit.
    #[inline]
    pub const fn with_d23_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D23_ENABLE) | ((enabled as u32) << 20))
    }
    /// Returns the PLL1 divider update enable bit.
    #[inline]
    pub const fn is_divider_update_enabled(self) -> bool {
        self.0 & Self::DIVIDER_UPDATE_ENABLE != 0
    }
    /// Sets the PLL1 divider update enable bit.
    #[inline]
    pub const fn with_divider_update_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::DIVIDER_UPDATE_ENABLE) | ((enabled as u32) << 21))
    }
    /// Returns the reference-buffer software override enable bit.
    #[inline]
    pub const fn is_reference_buffer_enabled(self) -> bool {
        self.0 & Self::REFERENCE_BUFFER_ENABLE != 0
    }
    /// Selects software enable instead of hardware control for the reference buffer.
    #[inline]
    pub const fn with_reference_buffer_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::REFERENCE_BUFFER_ENABLE) | ((enabled as u32) << 22))
    }
    /// Returns the bandgap enable bit.
    #[inline]
    pub const fn is_bandgap_enabled(self) -> bool {
        self.0 & Self::BANDGAP_ENABLE != 0
    }
    /// Sets the bandgap enable bit.
    #[inline]
    pub const fn with_bandgap_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::BANDGAP_ENABLE) | ((enabled as u32) << 23))
    }
    /// Returns the monitor-divider encoding.
    #[inline]
    pub const fn monitor_divider(self) -> u8 {
        ((self.0 & Self::MONITOR_DIVIDER_MASK) >> Self::MONITOR_DIVIDER_SHIFT) as u8
    }
    /// Sets the monitor-divider encoding, panicking unless it is in 0..=3.
    #[inline]
    pub const fn with_monitor_divider(self, value: u8) -> Self {
        assert!(value <= 3);
        Self(
            (self.0 & !Self::MONITOR_DIVIDER_MASK)
                | ((value as u32) << Self::MONITOR_DIVIDER_SHIFT),
        )
    }
    /// Returns the bandgap temperature-coefficient encoding.
    #[inline]
    pub const fn bandgap_temperature_coefficient(self) -> u8 {
        ((self.0 & Self::BANDGAP_TEMPERATURE_COEFFICIENT_MASK)
            >> Self::BANDGAP_TEMPERATURE_COEFFICIENT_SHIFT) as u8
    }
    /// Sets the bandgap temperature-coefficient encoding, panicking unless it is in 0..=3.
    #[inline]
    pub const fn with_bandgap_temperature_coefficient(self, value: u8) -> Self {
        assert!(value <= 3);
        Self(
            (self.0 & !Self::BANDGAP_TEMPERATURE_COEFFICIENT_MASK)
                | ((value as u32) << Self::BANDGAP_TEMPERATURE_COEFFICIENT_SHIFT),
        )
    }
    /// Returns the bandgap output encoding.
    #[inline]
    pub const fn bandgap_output(self) -> u8 {
        ((self.0 & Self::BANDGAP_OUTPUT_MASK) >> Self::BANDGAP_OUTPUT_SHIFT) as u8
    }
    /// Sets the bandgap output encoding, panicking unless it is in 0..=7.
    #[inline]
    pub const fn with_bandgap_output(self, value: u8) -> Self {
        assert!(value <= 7);
        Self((self.0 & !Self::BANDGAP_OUTPUT_MASK) | ((value as u32) << Self::BANDGAP_OUTPUT_SHIFT))
    }
}

#[cfg(test)]
mod tests {
    use super::{Pll1SoftwareControl2 as P, RegisterBlock};
    use core::mem::{align_of, offset_of, size_of};
    #[test]
    fn pll1_values_and_gate_masks() {
        assert_eq!(size_of::<P>(), 4);
        assert_eq!(align_of::<P>(), 4);
        for bits in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
            let value = P::from_bits(bits);
            for enabled in [false, true] {
                for (shift, updated, read) in [
                    (
                        0,
                        value.with_d64_enabled(enabled),
                        P::is_d64_enabled as fn(P) -> bool,
                    ),
                    (1, value.with_d2_enabled(enabled), P::is_d2_enabled),
                    (2, value.with_d3_enabled(enabled), P::is_d3_enabled),
                    (3, value.with_d4_enabled(enabled), P::is_d4_enabled),
                    (4, value.with_d5_enabled(enabled), P::is_d5_enabled),
                    (5, value.with_d6_enabled(enabled), P::is_d6_enabled),
                    (6, value.with_d7_enabled(enabled), P::is_d7_enabled),
                    (7, value.with_d8_enabled(enabled), P::is_d8_enabled),
                    (8, value.with_adc_enabled(enabled), P::is_adc_enabled),
                    (9, value.with_dac_enabled(enabled), P::is_dac_enabled),
                    (10, value.with_d10_enabled(enabled), P::is_d10_enabled),
                    (11, value.with_d100_enabled(enabled), P::is_d100_enabled),
                    (
                        12,
                        value.with_analog_test_enabled(enabled),
                        P::is_analog_test_enabled,
                    ),
                    (
                        13,
                        value.with_clock_test_enabled(enabled),
                        P::is_clock_test_enabled,
                    ),
                    (
                        14,
                        value.with_digital_test_enabled(enabled),
                        P::is_digital_test_enabled,
                    ),
                    (15, value.with_d11_enabled(enabled), P::is_d11_enabled),
                    (16, value.with_d13_enabled(enabled), P::is_d13_enabled),
                    (
                        19,
                        value.with_monitor_enabled(enabled),
                        P::is_monitor_enabled,
                    ),
                    (20, value.with_d23_enabled(enabled), P::is_d23_enabled),
                    (
                        21,
                        value.with_divider_update_enabled(enabled),
                        P::is_divider_update_enabled,
                    ),
                    (
                        22,
                        value.with_reference_buffer_enabled(enabled),
                        P::is_reference_buffer_enabled,
                    ),
                    (
                        23,
                        value.with_bandgap_enabled(enabled),
                        P::is_bandgap_enabled,
                    ),
                ] {
                    assert_eq!(read(updated), enabled);
                    assert_eq!(
                        updated.bits(),
                        (bits & !(1 << shift)) | ((enabled as u32) << shift)
                    );
                }
            }
            for field in 0..=3 {
                let updated = value.with_monitor_divider(field);
                assert_eq!(updated.monitor_divider(), field);
                assert_eq!(updated.bits(), (bits & !(3 << 17)) | ((field as u32) << 17));
            }
            for field in 0..=3 {
                let updated = value.with_bandgap_temperature_coefficient(field);
                assert_eq!(updated.bandgap_temperature_coefficient(), field);
                assert_eq!(updated.bits(), (bits & !(3 << 24)) | ((field as u32) << 24));
            }
            for field in 0..=7 {
                let updated = value.with_bandgap_output(field);
                assert_eq!(updated.bandgap_output(), field);
                assert_eq!(updated.bits(), (bits & !(7 << 26)) | ((field as u32) << 26));
            }
        }
    }

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll1_software_control1), 0x100);
        assert_eq!(offset_of!(RegisterBlock, pll1_software_control2), 0x104);
        assert_eq!(offset_of!(RegisterBlock, pll1_software_control3), 0x108);
        assert_eq!(offset_of!(RegisterBlock, pll2_software_control1), 0x118);
        assert_eq!(offset_of!(RegisterBlock, pll2_software_control2), 0x11c);
        assert_eq!(offset_of!(RegisterBlock, pll2_software_control3), 0x120);
        assert_eq!(offset_of!(RegisterBlock, pll3_software_control1), 0x124);
        assert_eq!(offset_of!(RegisterBlock, pll3_software_control2), 0x128);
        assert_eq!(offset_of!(RegisterBlock, pll3_software_control3), 0x12c);
        assert_eq!(size_of::<RegisterBlock>(), 0x1000);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
