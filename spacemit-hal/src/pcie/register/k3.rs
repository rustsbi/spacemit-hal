//! K3 PCIe link-management registers.

use crate::register::{RC, RW1C};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/pci/controller/dwc/pcie-spacemit.c

/// K3 PCIe link-management registers.
#[repr(C)]
pub struct RegisterBlock {
    /// interrupt control.
    pub interrupt_control: RW<u32>,
    /// link status.
    pub link_status: RO<u32>,
    /// intx status.
    pub intx_status: RW1C<u32>,
    /// intx enable.
    pub intx_enable: RW<u32>,
    /// interrupt status.
    pub interrupt_status: RW1C<u32>,
    /// interrupt enable.
    pub interrupt_enable: RW<u32>,
    /// message interrupt status.
    pub message_interrupt_status: RW1C<u32>,
    /// message interrupt enable.
    pub message_interrupt_enable: RW<u32>,
    _padding_0x020: [u32; 24],
    /// msi receive control.
    pub msi_receive_control: RW<u32>,
    /// msi receive address.
    pub msi_receive_address: RW<u32>,
    /// msix monitor mask.
    pub msix_monitor_mask: RW<u32>,
    /// msix monitor base.
    pub msix_monitor_base: RW<u32>,
    _padding_0x090: [u32; 8],
    /// monitor fifo data0.
    pub monitor_fifo_data0: RC<u32>,
    /// monitor fifo data1.
    pub monitor_fifo_data1: RC<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, interrupt_control), 0x000);
        assert_eq!(offset_of!(RegisterBlock, link_status), 0x004);
        assert_eq!(offset_of!(RegisterBlock, intx_status), 0x008);
        assert_eq!(offset_of!(RegisterBlock, intx_enable), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x010);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x014);
        assert_eq!(offset_of!(RegisterBlock, message_interrupt_status), 0x018);
        assert_eq!(offset_of!(RegisterBlock, message_interrupt_enable), 0x01c);
        assert_eq!(offset_of!(RegisterBlock, msi_receive_control), 0x080);
        assert_eq!(offset_of!(RegisterBlock, msi_receive_address), 0x084);
        assert_eq!(offset_of!(RegisterBlock, msix_monitor_mask), 0x088);
        assert_eq!(offset_of!(RegisterBlock, msix_monitor_base), 0x08c);
        assert_eq!(offset_of!(RegisterBlock, monitor_fifo_data0), 0x0b0);
        assert_eq!(offset_of!(RegisterBlock, monitor_fifo_data1), 0x0b4);
        assert_eq!(size_of::<RegisterBlock>(), 0x0b8);
    }
}
