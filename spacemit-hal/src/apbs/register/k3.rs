//! K3 PLL register layout.

use volatile_register::{RO, RW};

// Offsets: include/soc/spacemit/k3-syscon.h; extent: k3.dtsi.
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k3-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi
// Gates: https://github.com/torvalds/linux/blob/master/drivers/clk/spacemit/ccu-k3.c
// Fields: K3 User Manual, Clock & Reset, APB_SPARE2_REG.
// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/17_clock_reset.md
// PHY calibration: https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/14_connectivity/usb.md

/// K3 PLL registers.
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
    /// PLL4 software control 1.
    pub pll4_software_control1: RW<u32>,
    /// PLL4 software control 2.
    pub pll4_software_control2: RW<u32>,
    /// PLL4 software control 3.
    pub pll4_software_control3: RW<u32>,
    /// PLL5 software control 1.
    pub pll5_software_control1: RW<u32>,
    /// PLL5 software control 2.
    pub pll5_software_control2: RW<u32>,
    /// PLL5 software control 3.
    pub pll5_software_control3: RW<u32>,
    /// PLL6 software control 1.
    pub pll6_software_control1: RW<u32>,
    /// PLL6 software control 2.
    pub pll6_software_control2: RW<u32>,
    /// PLL6 software control 3.
    pub pll6_software_control3: RW<u32>,
    _padding_0x154: [u32; 1],
    /// PLL7 software control 1.
    pub pll7_software_control1: RW<u32>,
    /// PLL7 software control 2.
    pub pll7_software_control2: RW<u32>,
    /// PLL7 software control 3.
    pub pll7_software_control3: RW<u32>,
    _padding_0x164: [u32; 5],
    /// USB3/PCIe PHY calibration control.
    pub phy_calibration_control: RW<u32>,
    /// USB3/PCIe PHY calibration status.
    pub phy_calibration_status: RO<u32>,
    /// PLL8 software control 1.
    pub pll8_software_control1: RW<u32>,
    /// PLL8 software control 2.
    pub pll8_software_control2: RW<u32>,
    /// PLL8 software control 3.
    pub pll8_software_control3: RW<u32>,
    _padding_0x18c: [u32; 16285],
}

/// K3 PLL1 configuration and output clock gates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Pll1SoftwareControl2(u32);

impl Pll1SoftwareControl2 {
    const D1_ENABLE: u32 = 1 << 0;
    const D2_ENABLE: u32 = 1 << 1;
    const D3_ENABLE: u32 = 1 << 2;
    const D4_ENABLE: u32 = 1 << 3;
    const D5_ENABLE: u32 = 1 << 4;
    const D6_ENABLE: u32 = 1 << 5;
    const D7_ENABLE: u32 = 1 << 6;
    const D8_ENABLE: u32 = 1 << 7;
    const PLL_ENABLE: u32 = 1 << 16;
    const MONITOR_ENABLE: u32 = 1 << 17;
    const D10_ENABLE: u32 = 1 << 21;
    const DX_ENABLE: u32 = 1 << 22;
    const ANALOG_TEST_ENABLE: u32 = 1 << 28;
    const DIGITAL_TEST_ENABLE: u32 = 1 << 29;
    const CLOCK_TEST_ENABLE: u32 = 1 << 30;
    const D64_ENABLE: u32 = 1 << 31;
    const REG0_SHIFT: u32 = 8;
    const REG0_MASK: u32 = 0xff << Self::REG0_SHIFT;
    const MONITOR_DIVIDER_SHIFT: u32 = 18;
    const MONITOR_DIVIDER_MASK: u32 = 0x7 << Self::MONITOR_DIVIDER_SHIFT;
    const DX_DIVIDER_SHIFT: u32 = 23;
    const DX_DIVIDER_MASK: u32 = 0x1f << Self::DX_DIVIDER_SHIFT;

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
    /// Returns whether the PLL1 / 1 output is enabled.
    #[inline]
    pub const fn is_d1_enabled(self) -> bool {
        self.0 & Self::D1_ENABLE != 0
    }
    /// Sets the PLL1 / 1 output enable bit.
    #[inline]
    pub const fn with_d1_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D1_ENABLE) | enabled as u32)
    }
    /// Returns whether the PLL1 / 2 output is enabled.
    #[inline]
    pub const fn is_d2_enabled(self) -> bool {
        self.0 & Self::D2_ENABLE != 0
    }
    /// Sets the PLL1 / 2 output enable bit.
    #[inline]
    pub const fn with_d2_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D2_ENABLE) | ((enabled as u32) << 1))
    }
    /// Returns whether the PLL1 / 3 output is enabled.
    #[inline]
    pub const fn is_d3_enabled(self) -> bool {
        self.0 & Self::D3_ENABLE != 0
    }
    /// Sets the PLL1 / 3 output enable bit.
    #[inline]
    pub const fn with_d3_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D3_ENABLE) | ((enabled as u32) << 2))
    }
    /// Returns whether the PLL1 / 4 output is enabled.
    #[inline]
    pub const fn is_d4_enabled(self) -> bool {
        self.0 & Self::D4_ENABLE != 0
    }
    /// Sets the PLL1 / 4 output enable bit.
    #[inline]
    pub const fn with_d4_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D4_ENABLE) | ((enabled as u32) << 3))
    }
    /// Returns whether the PLL1 / 5 output is enabled.
    #[inline]
    pub const fn is_d5_enabled(self) -> bool {
        self.0 & Self::D5_ENABLE != 0
    }
    /// Sets the PLL1 / 5 output enable bit.
    #[inline]
    pub const fn with_d5_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D5_ENABLE) | ((enabled as u32) << 4))
    }
    /// Returns whether the PLL1 / 6 output is enabled.
    #[inline]
    pub const fn is_d6_enabled(self) -> bool {
        self.0 & Self::D6_ENABLE != 0
    }
    /// Sets the PLL1 / 6 output enable bit.
    #[inline]
    pub const fn with_d6_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D6_ENABLE) | ((enabled as u32) << 5))
    }
    /// Returns whether the PLL1 / 7 output is enabled.
    #[inline]
    pub const fn is_d7_enabled(self) -> bool {
        self.0 & Self::D7_ENABLE != 0
    }
    /// Sets the PLL1 / 7 output enable bit.
    #[inline]
    pub const fn with_d7_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D7_ENABLE) | ((enabled as u32) << 6))
    }
    /// Returns whether the PLL1 / 8 output is enabled.
    #[inline]
    pub const fn is_d8_enabled(self) -> bool {
        self.0 & Self::D8_ENABLE != 0
    }
    /// Sets the PLL1 / 8 output enable bit.
    #[inline]
    pub const fn with_d8_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D8_ENABLE) | ((enabled as u32) << 7))
    }
    /// Returns whether the PLL1 is enabled.
    #[inline]
    pub const fn is_pll_enabled(self) -> bool {
        self.0 & Self::PLL_ENABLE != 0
    }
    /// Sets the PLL1 enable bit.
    #[inline]
    pub const fn with_pll_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::PLL_ENABLE) | ((enabled as u32) << 16))
    }
    /// Returns whether the PLL1 monitor is enabled.
    #[inline]
    pub const fn is_monitor_enabled(self) -> bool {
        self.0 & Self::MONITOR_ENABLE != 0
    }
    /// Sets the PLL1 monitor enable bit.
    #[inline]
    pub const fn with_monitor_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::MONITOR_ENABLE) | ((enabled as u32) << 17))
    }
    /// Returns whether the PLL1 / 10 audio output is enabled.
    #[inline]
    pub const fn is_d10_enabled(self) -> bool {
        self.0 & Self::D10_ENABLE != 0
    }
    /// Sets the PLL1 / 10 audio output enable bit.
    #[inline]
    pub const fn with_d10_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D10_ENABLE) | ((enabled as u32) << 21))
    }
    /// Returns whether the PLL1 programmable-divider output is enabled.
    #[inline]
    pub const fn is_dx_enabled(self) -> bool {
        self.0 & Self::DX_ENABLE != 0
    }
    /// Sets the PLL1 programmable-divider output enable bit.
    #[inline]
    pub const fn with_dx_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::DX_ENABLE) | ((enabled as u32) << 22))
    }
    /// Returns whether the PLL1 analog test is enabled.
    #[inline]
    pub const fn is_analog_test_enabled(self) -> bool {
        self.0 & Self::ANALOG_TEST_ENABLE != 0
    }
    /// Sets the PLL1 analog test enable bit.
    #[inline]
    pub const fn with_analog_test_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::ANALOG_TEST_ENABLE) | ((enabled as u32) << 28))
    }
    /// Returns whether the PLL1 digital test is enabled.
    #[inline]
    pub const fn is_digital_test_enabled(self) -> bool {
        self.0 & Self::DIGITAL_TEST_ENABLE != 0
    }
    /// Sets the PLL1 digital test enable bit.
    #[inline]
    pub const fn with_digital_test_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::DIGITAL_TEST_ENABLE) | ((enabled as u32) << 29))
    }
    /// Returns whether the PLL1 clock test is enabled.
    #[inline]
    pub const fn is_clock_test_enabled(self) -> bool {
        self.0 & Self::CLOCK_TEST_ENABLE != 0
    }
    /// Sets the PLL1 clock test enable bit.
    #[inline]
    pub const fn with_clock_test_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::CLOCK_TEST_ENABLE) | ((enabled as u32) << 30))
    }
    /// Returns whether the PLL1 / 64 output is enabled.
    #[inline]
    pub const fn is_d64_enabled(self) -> bool {
        self.0 & Self::D64_ENABLE != 0
    }
    /// Sets the PLL1 / 64 output enable bit.
    #[inline]
    pub const fn with_d64_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D64_ENABLE) | ((enabled as u32) << 31))
    }
    /// Returns the PLL1 REG0 configuration byte.
    #[inline]
    pub const fn reg0(self) -> u8 {
        ((self.0 & Self::REG0_MASK) >> Self::REG0_SHIFT) as u8
    }
    /// Sets the PLL1 REG0 configuration byte.
    #[inline]
    pub const fn with_reg0(self, value: u8) -> Self {
        Self((self.0 & !Self::REG0_MASK) | ((value as u32) << Self::REG0_SHIFT))
    }
    /// Returns the three-bit monitor-divider encoding.
    #[inline]
    pub const fn monitor_divider(self) -> u8 {
        ((self.0 & Self::MONITOR_DIVIDER_MASK) >> Self::MONITOR_DIVIDER_SHIFT) as u8
    }
    /// Sets the monitor-divider encoding, panicking unless it is in 0..=7.
    #[inline]
    pub const fn with_monitor_divider(self, divider: u8) -> Self {
        assert!(divider <= 7);
        Self(
            (self.0 & !Self::MONITOR_DIVIDER_MASK)
                | ((divider as u32) << Self::MONITOR_DIVIDER_SHIFT),
        )
    }
    /// Returns the programmable output divisor in 1..=32.
    #[inline]
    pub const fn dx_divider(self) -> u8 {
        ((self.0 & Self::DX_DIVIDER_MASK) >> Self::DX_DIVIDER_SHIFT) as u8 + 1
    }
    /// Sets the programmable output divisor, panicking unless it is in 1..=32.
    #[inline]
    pub const fn with_dx_divider(self, divider: u8) -> Self {
        assert!(divider >= 1 && divider <= 32);
        Self((self.0 & !Self::DX_DIVIDER_MASK) | (((divider - 1) as u32) << Self::DX_DIVIDER_SHIFT))
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
                        value.with_d1_enabled(enabled),
                        P::is_d1_enabled as fn(P) -> bool,
                    ),
                    (1, value.with_d2_enabled(enabled), P::is_d2_enabled),
                    (2, value.with_d3_enabled(enabled), P::is_d3_enabled),
                    (3, value.with_d4_enabled(enabled), P::is_d4_enabled),
                    (4, value.with_d5_enabled(enabled), P::is_d5_enabled),
                    (5, value.with_d6_enabled(enabled), P::is_d6_enabled),
                    (6, value.with_d7_enabled(enabled), P::is_d7_enabled),
                    (7, value.with_d8_enabled(enabled), P::is_d8_enabled),
                    (16, value.with_pll_enabled(enabled), P::is_pll_enabled),
                    (
                        17,
                        value.with_monitor_enabled(enabled),
                        P::is_monitor_enabled,
                    ),
                    (21, value.with_d10_enabled(enabled), P::is_d10_enabled),
                    (22, value.with_dx_enabled(enabled), P::is_dx_enabled),
                    (
                        28,
                        value.with_analog_test_enabled(enabled),
                        P::is_analog_test_enabled,
                    ),
                    (
                        29,
                        value.with_digital_test_enabled(enabled),
                        P::is_digital_test_enabled,
                    ),
                    (
                        30,
                        value.with_clock_test_enabled(enabled),
                        P::is_clock_test_enabled,
                    ),
                    (31, value.with_d64_enabled(enabled), P::is_d64_enabled),
                ] {
                    assert_eq!(read(updated), enabled);
                    assert_eq!(
                        updated.bits(),
                        (bits & !(1 << shift)) | ((enabled as u32) << shift)
                    );
                }
            }
            for byte in 0..=u8::MAX {
                let reg0 = value.with_reg0(byte);
                assert_eq!(reg0.reg0(), byte);
                assert_eq!(reg0.bits(), (bits & !0xff00) | ((byte as u32) << 8));
            }
            for divider in 0..=7 {
                let monitor = value.with_monitor_divider(divider);
                assert_eq!(monitor.monitor_divider(), divider);
                assert_eq!(
                    monitor.bits(),
                    (bits & !(7 << 18)) | ((divider as u32) << 18)
                );
            }
            for divider in 1..=32 {
                let output = value.with_dx_divider(divider);
                assert_eq!(output.dx_divider(), divider);
                assert_eq!(
                    output.bits(),
                    (bits & !(31 << 23)) | (((divider - 1) as u32) << 23)
                );
            }
        }
    }

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll1_software_control2), 0x104);
        assert_eq!(offset_of!(RegisterBlock, phy_calibration_control), 0x178);
        assert_eq!(offset_of!(RegisterBlock, phy_calibration_status), 0x17c);
        assert_eq!(offset_of!(RegisterBlock, pll1_software_control1), 0x100);
        assert_eq!(offset_of!(RegisterBlock, pll1_software_control3), 0x108);
        assert_eq!(offset_of!(RegisterBlock, pll2_software_control1), 0x118);
        assert_eq!(offset_of!(RegisterBlock, pll2_software_control2), 0x11c);
        assert_eq!(offset_of!(RegisterBlock, pll2_software_control3), 0x120);
        assert_eq!(offset_of!(RegisterBlock, pll3_software_control1), 0x124);
        assert_eq!(offset_of!(RegisterBlock, pll3_software_control2), 0x128);
        assert_eq!(offset_of!(RegisterBlock, pll3_software_control3), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, pll4_software_control1), 0x130);
        assert_eq!(offset_of!(RegisterBlock, pll4_software_control2), 0x134);
        assert_eq!(offset_of!(RegisterBlock, pll4_software_control3), 0x138);
        assert_eq!(offset_of!(RegisterBlock, pll5_software_control1), 0x13c);
        assert_eq!(offset_of!(RegisterBlock, pll5_software_control2), 0x140);
        assert_eq!(offset_of!(RegisterBlock, pll5_software_control3), 0x144);
        assert_eq!(offset_of!(RegisterBlock, pll6_software_control1), 0x148);
        assert_eq!(offset_of!(RegisterBlock, pll6_software_control2), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, pll6_software_control3), 0x150);
        assert_eq!(offset_of!(RegisterBlock, pll7_software_control1), 0x158);
        assert_eq!(offset_of!(RegisterBlock, pll7_software_control2), 0x15c);
        assert_eq!(offset_of!(RegisterBlock, pll7_software_control3), 0x160);
        assert_eq!(offset_of!(RegisterBlock, pll8_software_control1), 0x180);
        assert_eq!(offset_of!(RegisterBlock, pll8_software_control2), 0x184);
        assert_eq!(offset_of!(RegisterBlock, pll8_software_control3), 0x188);
        assert_eq!(size_of::<RegisterBlock>(), 0x10000);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
