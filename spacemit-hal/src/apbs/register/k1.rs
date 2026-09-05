//! K1 PLL register layout.

use volatile_register::RW;

// Offsets: include/soc/spacemit/k1-syscon.h; extent: k1.dtsi.
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k1-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k1.dtsi
// Gates: https://github.com/torvalds/linux/blob/master/drivers/clk/spacemit/ccu-k1.c

/// K1 PLL registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0x000: [u32; 65],
    /// PLL1 software control 2.
    pub pll1_software_control2: RW<Pll1SoftwareControl2>,
    _reserved_0x108: [u32; 958],
}

/// K1 PLL1 output clock gates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Pll1SoftwareControl2(u32);

impl Pll1SoftwareControl2 {
    const D4_ENABLE: u32 = 1 << 3;
    const D5_ENABLE: u32 = 1 << 4;
    const D23_ENABLE: u32 = 1 << 20;
    /// Creates a register value from raw bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
    /// Returns the raw register bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
    /// Returns whether the PLL1 / 4 output is enabled.
    pub const fn is_d4_enabled(self) -> bool {
        self.0 & Self::D4_ENABLE != 0
    }
    /// Sets the PLL1 / 4 output gate in this value.
    pub const fn with_d4_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D4_ENABLE) | ((enabled as u32) << 3))
    }
    /// Returns whether the PLL1 / 5 output is enabled.
    pub const fn is_d5_enabled(self) -> bool {
        self.0 & Self::D5_ENABLE != 0
    }
    /// Sets the PLL1 / 5 output gate in this value.
    pub const fn with_d5_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D5_ENABLE) | ((enabled as u32) << 4))
    }
    /// Returns whether the PLL1 / 23 output is enabled.
    pub const fn is_d23_enabled(self) -> bool {
        self.0 & Self::D23_ENABLE != 0
    }
    /// Sets the PLL1 / 23 output gate in this value.
    pub const fn with_d23_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::D23_ENABLE) | ((enabled as u32) << 20))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};
    #[test]
    fn pll1_values_and_gate_masks() {
        assert_eq!(size_of::<Pll1SoftwareControl2>(), 4);
        assert_eq!(align_of::<Pll1SoftwareControl2>(), 4);
        for bits in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
            for enabled in [false, true] {
                let value = Pll1SoftwareControl2::from_bits(bits).with_d4_enabled(enabled);
                assert_eq!(value.is_d4_enabled(), enabled);
                assert_eq!(value.bits(), (bits & !(1 << 3)) | ((enabled as u32) << 3));
                let value = Pll1SoftwareControl2::from_bits(bits).with_d5_enabled(enabled);
                assert_eq!(value.is_d5_enabled(), enabled);
                assert_eq!(value.bits(), (bits & !(1 << 4)) | ((enabled as u32) << 4));
                let value = Pll1SoftwareControl2::from_bits(bits).with_d23_enabled(enabled);
                assert_eq!(value.is_d23_enabled(), enabled);
                assert_eq!(value.bits(), (bits & !(1 << 20)) | ((enabled as u32) << 20));
            }
        }
    }

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll1_software_control2), 0x104);
        assert_eq!(size_of::<RegisterBlock>(), 0x1000);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
