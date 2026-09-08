//! K1/M1 PCIe link-management registers.

use crate::register::RW1C;
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/pci/controller/dwc/pcie-spacemit-k1.c

/// K1/M1 PCIe link-management registers.
#[repr(C)]
pub struct RegisterBlock {
    /// interrupt control.
    pub interrupt_control: RW<u32>,
    /// link status.
    pub link_status: RO<u32>,
    _padding_0x008: [u32; 2],
    /// interrupt status.
    pub interrupt_status: RW1C<u32>,
    /// interrupt enable.
    pub interrupt_enable: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, interrupt_control), 0x000);
        assert_eq!(offset_of!(RegisterBlock, link_status), 0x004);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x010);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x014);
        assert_eq!(size_of::<RegisterBlock>(), 0x018);
    }
}
