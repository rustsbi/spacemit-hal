//! K3 PCIe and USB3 PHY registers.

use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/14_connectivity/usb.md
// Section 14.2.6.3: documented combo-PHY prefix.

/// K3 PCIe and USB3 PHY registers.
#[repr(C)]
pub struct RegisterBlock {
    /// PHY version.
    pub version: RO<u32>,
    _padding_0x004: [u32; 1],
    /// Reference clock, lane clocks, overrides and PLL status.
    pub clock_config: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, version), 0x0);
        assert_eq!(offset_of!(RegisterBlock, clock_config), 0x8);
        assert_eq!(size_of::<RegisterBlock>(), 0xc);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
