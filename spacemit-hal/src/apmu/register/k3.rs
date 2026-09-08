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
    _padding_0x000: [u32; 9],
    /// CSI clock and reset control.
    pub csi_clock_reset: RWNoModify<u32>,
    _padding_0x028: [u32; 4],
    /// ISP clock and reset control.
    pub isp_clock_reset: RWNoModify<u32>,
    _padding_0x03c: [u32; 1],
    /// PMU clock and reset control.
    pub pmu_clock_reset: RWNoModify<u32>,
    /// LCD1 clock and reset control.
    pub lcd1_clock_reset: RWNoModify<u32>,
    /// LCD_SPI clock and reset control.
    pub lcd_spi_clock_reset: RWNoModify<u32>,
    /// LCD2 clock and reset control.
    pub lcd2_clock_reset: RWNoModify<u32>,
    /// CCIC clock and reset control.
    pub ccic_clock_reset: RWNoModify<u32>,
    /// SDH0 clock and reset control.
    pub sdh0_clock_reset: RWNoModify<u32>,
    /// SDH1 clock and reset control.
    pub sdh1_clock_reset: RWNoModify<u32>,
    /// USB controller clocks and resets.
    pub usb_clock_reset: RW<u32>,
    /// QSPI clock/reset control with a self-clearing frequency-change request.
    pub qspi_clock_reset: RW<QspiClockReset>,
    /// DMA clock and reset control.
    pub dma_clock_reset: RWNoModify<u32>,
    /// AES clock and reset control.
    pub aes_clock_reset: RWNoModify<u32>,
    /// MCB clock and reset control.
    pub mcb_clock_reset: RWNoModify<u32>,
    _padding_0x070: [u32; 3],
    /// USB2, SD and debug wakeup control and acknowledgement.
    pub usb_sd_wakeup: RWNoModify<u32>,
    _padding_0x080: [u32; 9],
    /// VPU clock and reset control.
    pub vpu_clock_reset: RWNoModify<u32>,
    _padding_0x0a8: [u32; 1],
    /// DTC clock and reset control.
    pub dtc_clock_reset: RWNoModify<u32>,
    _padding_0x0b0: [u32; 7],
    /// GPU clock and reset control.
    pub gpu_clock_reset: RWNoModify<u32>,
    _padding_0x0d0: [u32; 4],
    /// SDH2 clock and reset control.
    pub sdh2_clock_reset: RWNoModify<u32>,
    _padding_0x0e4: [u32; 1],
    /// MEMORY clock and reset control.
    pub memory_clock_reset: RWNoModify<u32>,
    _padding_0x0ec: [u32; 5],
    /// AP clock and reset control.
    pub ap_clock_reset: RWNoModify<u32>,
    /// EM clock and reset control.
    pub em_clock_reset: RWNoModify<u32>,
    _padding_0x108: [u32; 2],
    /// USB Type-C orientation and interrupt control.
    pub usb_phy_control: RWNoModify<u32>,
    _padding_0x114: [u32; 1],
    /// USB PHY line and connection status.
    pub usb_phy_status: RO<u32>,
    /// UCIE clock and reset control.
    pub ucie_clock_reset: RWNoModify<u32>,
    _padding_0x120: [u32; 3],
    /// Cluster 0 hart wakeup requests.
    pub cluster0_wakeup: [WO<u32>; 4],
    _padding_0x13c: [u32; 4],
    /// RCPU clock and reset control.
    pub rcpu_clock_reset: RWNoModify<u32>,
    _padding_0x150: [u32; 2],
    /// TOP_DCLK clock and reset control.
    pub top_dclk_clock_reset: RWNoModify<u32>,
    _padding_0x15c: [u32; 27],
    /// PCIE_C clock and reset control.
    pub pcie_c_clock_reset: RWNoModify<u32>,
    _padding_0x1cc: [u32; 1],
    /// PCIE_B clock and reset control.
    pub pcie_b_clock_reset: RWNoModify<u32>,
    _padding_0x1d4: [u32; 1],
    /// PCIe/USB combo routing, IOMMU bypass and memory power control.
    pub pcie_subsystem_management: RW<u32>,
    _padding_0x1dc: [u32; 1],
    /// PCIE_D clock and reset control.
    pub pcie_d_clock_reset: RWNoModify<u32>,
    _padding_0x1e4: [u32; 1],
    /// PCIE_E clock and reset control.
    pub pcie_e_clock_reset: RWNoModify<u32>,
    _padding_0x1ec: [u32; 1],
    /// PCIE_A clock and reset control.
    pub pcie_a_clock_reset: RWNoModify<u32>,
    _padding_0x1f4: [u32; 5],
    /// CPU_CLUSTER3 clock and reset control.
    pub cpu_cluster3_clock_reset: RWNoModify<u32>,
    _padding_0x20c: [u32; 8],
    /// Cluster 3 hart wakeup requests.
    pub cluster3_wakeup: [WO<u32>; 4],
    /// EDP clock and reset control.
    pub edp_clock_reset: RWNoModify<u32>,
    /// ESPI clock and reset control.
    pub espi_clock_reset: RWNoModify<u32>,
    _padding_0x244: [u32; 1],
    /// GMAC2 clocks, reset and interface selection.
    pub gmac2_control: RW<u32>,
    /// GMAC2 receive and transmit delay lines.
    pub gmac2_delay: RW<u32>,
    _padding_0x250: [u32; 6],
    /// UFS clock and reset control.
    pub ufs_clock_reset: RWNoModify<u32>,
    /// LCD3 clock and reset control.
    pub lcd3_clock_reset: RWNoModify<u32>,
    /// LCD4 clock and reset control.
    pub lcd4_clock_reset: RWNoModify<u32>,
    /// LCD5 clock and reset control.
    pub lcd5_clock_reset: RWNoModify<u32>,
    _padding_0x278: [u32; 34],
    /// CCI550 clock and reset control.
    pub cci550_clock_reset: RWNoModify<u32>,
    _padding_0x304: [u32; 8],
    /// Cluster 1 hart wakeup requests.
    pub cluster1_wakeup: [WO<u32>; 4],
    _padding_0x334: [u32; 11],
    /// Cluster 2 hart wakeup requests.
    pub cluster2_wakeup: [WO<u32>; 4],
    _padding_0x370: [u32; 6],
    /// ACLK clock and reset control.
    pub aclk_clock_reset: RWNoModify<u32>,
    /// CPU_CLUSTER0 clock and reset control.
    pub cpu_cluster0_clock_reset: RWNoModify<u32>,
    /// CPU_CLUSTER1 clock and reset control.
    pub cpu_cluster1_clock_reset: RWNoModify<u32>,
    /// CPU_CLUSTER2 clock and reset control.
    pub cpu_cluster2_clock_reset: RWNoModify<u32>,
    _padding_0x398: [u32; 11],
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
    _padding_0x3f4: [u32; 1],
    /// SNR_ISIM clock and reset control.
    pub snr_isim_clock_reset: RWNoModify<u32>,
    _padding_0x3fc: [u32; 1],
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
        assert_eq!(offset_of!(RegisterBlock, csi_clock_reset), 0x24);
        assert_eq!(offset_of!(RegisterBlock, isp_clock_reset), 0x38);
        assert_eq!(offset_of!(RegisterBlock, pmu_clock_reset), 0x40);
        assert_eq!(offset_of!(RegisterBlock, lcd1_clock_reset), 0x44);
        assert_eq!(offset_of!(RegisterBlock, lcd_spi_clock_reset), 0x48);
        assert_eq!(offset_of!(RegisterBlock, lcd2_clock_reset), 0x4c);
        assert_eq!(offset_of!(RegisterBlock, ccic_clock_reset), 0x50);
        assert_eq!(offset_of!(RegisterBlock, sdh0_clock_reset), 0x54);
        assert_eq!(offset_of!(RegisterBlock, sdh1_clock_reset), 0x58);
        assert_eq!(offset_of!(RegisterBlock, dma_clock_reset), 0x64);
        assert_eq!(offset_of!(RegisterBlock, aes_clock_reset), 0x68);
        assert_eq!(offset_of!(RegisterBlock, mcb_clock_reset), 0x6c);
        assert_eq!(offset_of!(RegisterBlock, vpu_clock_reset), 0xa4);
        assert_eq!(offset_of!(RegisterBlock, dtc_clock_reset), 0xac);
        assert_eq!(offset_of!(RegisterBlock, gpu_clock_reset), 0xcc);
        assert_eq!(offset_of!(RegisterBlock, sdh2_clock_reset), 0xe0);
        assert_eq!(offset_of!(RegisterBlock, memory_clock_reset), 0xe8);
        assert_eq!(offset_of!(RegisterBlock, ap_clock_reset), 0x100);
        assert_eq!(offset_of!(RegisterBlock, em_clock_reset), 0x104);
        assert_eq!(offset_of!(RegisterBlock, ucie_clock_reset), 0x11c);
        assert_eq!(offset_of!(RegisterBlock, rcpu_clock_reset), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, top_dclk_clock_reset), 0x158);
        assert_eq!(offset_of!(RegisterBlock, edp_clock_reset), 0x23c);
        assert_eq!(offset_of!(RegisterBlock, ufs_clock_reset), 0x268);
        assert_eq!(offset_of!(RegisterBlock, lcd3_clock_reset), 0x26c);
        assert_eq!(offset_of!(RegisterBlock, lcd4_clock_reset), 0x270);
        assert_eq!(offset_of!(RegisterBlock, lcd5_clock_reset), 0x274);
        assert_eq!(offset_of!(RegisterBlock, cci550_clock_reset), 0x300);
        assert_eq!(offset_of!(RegisterBlock, aclk_clock_reset), 0x388);
        assert_eq!(offset_of!(RegisterBlock, cpu_cluster0_clock_reset), 0x38c);
        assert_eq!(offset_of!(RegisterBlock, cpu_cluster1_clock_reset), 0x390);
        assert_eq!(offset_of!(RegisterBlock, cpu_cluster2_clock_reset), 0x394);
        assert_eq!(offset_of!(RegisterBlock, cpu_cluster3_clock_reset), 0x208);
        assert_eq!(offset_of!(RegisterBlock, pcie_a_clock_reset), 0x1f0);
        assert_eq!(offset_of!(RegisterBlock, pcie_b_clock_reset), 0x1d0);
        assert_eq!(offset_of!(RegisterBlock, pcie_c_clock_reset), 0x1c8);
        assert_eq!(offset_of!(RegisterBlock, pcie_d_clock_reset), 0x1e0);
        assert_eq!(offset_of!(RegisterBlock, pcie_e_clock_reset), 0x1e8);
        assert_eq!(offset_of!(RegisterBlock, espi_clock_reset), 0x240);
        assert_eq!(offset_of!(RegisterBlock, snr_isim_clock_reset), 0x3f8);
        assert_eq!(size_of::<RegisterBlock>(), 0x400);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
