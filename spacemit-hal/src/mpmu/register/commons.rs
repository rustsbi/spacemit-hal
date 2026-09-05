//! Application clock-gate values.

// Linux ccu-k1.c and ccu-k3.c: MPMU_ACGR bit 6 gates PLL1 / 78.
// https://github.com/torvalds/linux/blob/master/drivers/clk/spacemit/ccu-k1.c
// https://github.com/torvalds/linux/blob/master/drivers/clk/spacemit/ccu-k3.c

/// Application clock gates shared by K1/M1 and K3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ApplicationClockGate(u32);

impl ApplicationClockGate {
    const PLL1_D78_ENABLE: u32 = 1 << 6;
    /// Creates a register value from raw bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
    /// Returns the raw register bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
    /// Returns whether the PLL1 / 78 output is enabled.
    pub const fn is_pll1_d78_enabled(self) -> bool {
        self.0 & Self::PLL1_D78_ENABLE != 0
    }
    /// Sets the PLL1 / 78 output gate in this value.
    pub const fn with_pll1_d78_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::PLL1_D78_ENABLE) | ((enabled as u32) << 6))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};
    #[test]
    fn gate_value_and_mask() {
        assert_eq!(size_of::<ApplicationClockGate>(), 4);
        assert_eq!(align_of::<ApplicationClockGate>(), 4);
        for bits in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
            for enabled in [false, true] {
                let value = ApplicationClockGate::from_bits(bits).with_pll1_d78_enabled(enabled);
                assert_eq!(value.is_pll1_d78_enabled(), enabled);
                assert_eq!(value.bits(), (bits & !64) | ((enabled as u32) << 6));
            }
        }
    }
}
