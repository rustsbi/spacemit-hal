//! K3 USB2 PHY registers.

use crate::register::RWNoModify;
use volatile_register::RW;

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/14_connectivity/usb.md
// Section 14.2.6.2: documented UTMI subset, not USB controller registers.

/// K3 USB2 PHY registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 8],
    /// Host disconnect acknowledgement.
    pub disconnect_clear: RWNoModify<u32>,
}

/// USB3 Port A UTMI overrides and disconnect control.
#[repr(C)]
pub struct PortARegisterBlock {
    _padding_0x000: [u32; 5],
    /// Port A VBUS overrides; unavailable on other ports.
    pub vbus_control: RW<u32>,
    _padding_0x018: [u32; 2],
    /// Host disconnect acknowledgement.
    pub disconnect_clear: RWNoModify<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, disconnect_clear), 0x20);
        assert_eq!(size_of::<RegisterBlock>(), 0x24);
        assert_eq!(align_of::<RegisterBlock>(), 4);
        assert_eq!(offset_of!(PortARegisterBlock, vbus_control), 0x14);
        assert_eq!(offset_of!(PortARegisterBlock, disconnect_clear), 0x20);
        assert_eq!(size_of::<PortARegisterBlock>(), 0x24);
        assert_eq!(align_of::<PortARegisterBlock>(), 4);
    }
}
