//! K3 APMU register layout.

use super::QspiClockReset;
use crate::register::RWNoModify;
use volatile_register::{RO, RW, WO};

// Offset: include/soc/spacemit/k3-syscon.h; window: k3.dtsi (0x400 bytes).
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k3-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi
// Wakeup offsets: vendor OpenSBI platform/generic/include/spacemit/k3/k3.h.
// USB: https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/14_connectivity/usb.md
// GMAC: https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/14_connectivity/ethernet_gmac.md
// GMAC0/1/2 offsets use APMU base 0xd4282800; the GMAC chapter's repeated base is a typo.

/// K3 APMU registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 23],
    /// USB controller clocks and resets.
    pub usb_clock_reset: RW<u32>,
    /// QSPI clock/reset control with a self-clearing frequency-change request.
    pub qspi_clock_reset: RW<QspiClockReset>,
    _padding_0x064: [u32; 6],
    /// USB2, SD and debug wakeup control and acknowledgement.
    pub usb_sd_wakeup: RWNoModify<u32>,
    _padding_0x080: [u32; 36],
    /// USB Type-C orientation and interrupt control.
    pub usb_phy_control: RWNoModify<u32>,
    _padding_0x114: [u32; 1],
    /// USB PHY line and connection status.
    pub usb_phy_status: RO<u32>,
    _padding_0x11c: [u32; 4],
    /// Cluster 0 hart wakeup requests.
    pub cluster0_wakeup: [WO<u32>; 4],
    _padding_0x13c: [u32; 39],
    /// PCIe/USB combo routing, IOMMU bypass and memory power control.
    pub pcie_subsystem_management: RW<u32>,
    _padding_0x1dc: [u32; 20],
    /// Cluster 3 hart wakeup requests.
    pub cluster3_wakeup: [WO<u32>; 4],
    _padding_0x23c: [u32; 3],
    /// GMAC2 clocks, reset and interface selection.
    pub gmac2_control: RW<u32>,
    /// GMAC2 receive and transmit delay lines.
    pub gmac2_delay: RW<u32>,
    _padding_0x250: [u32; 53],
    /// Cluster 1 hart wakeup requests.
    pub cluster1_wakeup: [WO<u32>; 4],
    _padding_0x334: [u32; 11],
    /// Cluster 2 hart wakeup requests.
    pub cluster2_wakeup: [WO<u32>; 4],
    _padding_0x370: [u32; 21],
    /// USB3 ports A..D wakeup control and acknowledgement.
    pub usb3_wakeup: [RWNoModify<u32>; 4],
    _padding_0x3d4: [u32; 4],
    /// GMAC0 clocks, reset and interface selection.
    pub gmac0_control: RW<u32>,
    /// GMAC0 receive and transmit delay lines.
    pub gmac0_delay: RW<u32>,
    /// GMAC1 clocks, reset and interface selection.
    pub gmac1_control: RW<u32>,
    /// GMAC1 receive and transmit delay lines.
    pub gmac1_delay: RW<u32>,
    _padding_0x3f4: [u32; 3],
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, usb_clock_reset), 0x05c);
        assert_eq!(offset_of!(RegisterBlock, qspi_clock_reset), 0x060);
        assert_eq!(offset_of!(RegisterBlock, usb_sd_wakeup), 0x07c);
        assert_eq!(offset_of!(RegisterBlock, usb_phy_control), 0x110);
        assert_eq!(offset_of!(RegisterBlock, usb_phy_status), 0x118);
        assert_eq!(offset_of!(RegisterBlock, pcie_subsystem_management), 0x1d8);
        assert_eq!(offset_of!(RegisterBlock, gmac2_control), 0x248);
        assert_eq!(offset_of!(RegisterBlock, gmac2_delay), 0x24c);
        assert_eq!(offset_of!(RegisterBlock, usb3_wakeup), 0x3c4);
        assert_eq!(offset_of!(RegisterBlock, gmac0_control), 0x3e4);
        assert_eq!(offset_of!(RegisterBlock, gmac0_delay), 0x3e8);
        assert_eq!(offset_of!(RegisterBlock, gmac1_control), 0x3ec);
        assert_eq!(offset_of!(RegisterBlock, gmac1_delay), 0x3f0);
        assert_eq!(offset_of!(RegisterBlock, cluster0_wakeup), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, cluster1_wakeup), 0x324);
        assert_eq!(offset_of!(RegisterBlock, cluster2_wakeup), 0x360);
        assert_eq!(offset_of!(RegisterBlock, cluster3_wakeup), 0x22c);
        assert_eq!(size_of::<RegisterBlock>(), 0x400);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
