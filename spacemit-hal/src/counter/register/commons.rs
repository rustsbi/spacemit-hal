//! Generic counter control values.

// Vendor K1 spl.c enables GEN_CNT by setting bit 0 without changing other bits.
// https://github.com/spacemit-com/uboot-2022.10/blob/1fa1ca64e9705a3650bcc7c21f6666949290830f/board/spacemit/k1-x/spl.c

/// Generic counter enable control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Control(u32);
impl Control {
    const ENABLE: u32 = 1;
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
    /// Returns whether the counter is enabled.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        self.0 & Self::ENABLE != 0
    }
    /// Sets the counter enable bit in this value.
    #[inline]
    pub const fn with_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::ENABLE) | (enabled as u32))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};
    #[test]
    fn control_value_and_mask() {
        assert_eq!(size_of::<Control>(), 4);
        assert_eq!(align_of::<Control>(), 4);
        for bits in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
            for enabled in [false, true] {
                let value = Control::from_bits(bits).with_enabled(enabled);
                assert_eq!(value.is_enabled(), enabled);
                assert_eq!(value.bits(), (bits & !1) | (enabled as u32));
            }
        }
    }
}
