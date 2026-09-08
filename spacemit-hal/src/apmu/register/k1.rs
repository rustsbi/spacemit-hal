//! K1/M1 APMU register layout.

use super::QspiClockReset;
use crate::register::RWNoModify;
use volatile_register::{RW, WO};

// Offset: include/soc/spacemit/k1-syscon.h; window: k1.dtsi (0x400 bytes).
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k1-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k1.dtsi
// Wakeup offsets: vendor OpenSBI platform/generic/include/spacemit/k1x/k1x_evb.h.

/// K1/M1 APMU registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 8],
    /// JPEG clock and reset control.
    pub jpeg_clock_reset: RWNoModify<u32>,
    /// CSI clock and reset control.
    pub csi_clock_reset: RWNoModify<u32>,
    _padding_0x028: [u32; 4],
    /// ISP clock and reset control.
    pub isp_clock_reset: RWNoModify<u32>,
    _padding_0x03c: [u32; 2],
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
    /// USB clock and reset control.
    pub usb_clock_reset: RWNoModify<u32>,
    /// QSPI clock/reset control with a self-clearing frequency-change request.
    pub qspi_clock_reset: RW<QspiClockReset>,
    /// DMA clock and reset control.
    pub dma_clock_reset: RWNoModify<u32>,
    /// AES clock and reset control.
    pub aes_clock_reset: RWNoModify<u32>,
    _padding_0x06c: [u32; 14],
    /// VPU clock and reset control.
    pub vpu_clock_reset: RWNoModify<u32>,
    _padding_0x0a8: [u32; 9],
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
    _padding_0x108: [u32; 9],
    /// Wakeup requests issued by cluster 0 harts.
    pub cluster0_wakeup: [WO<u32>; 4],
    _padding_0x13c: [u32; 4],
    /// AUDIO clock and reset control.
    pub audio_clock_reset: RWNoModify<u32>,
    _padding_0x150: [u32; 26],
    /// HDMI clock and reset control.
    pub hdmi_clock_reset: RWNoModify<u32>,
    _padding_0x1bc: [u32; 81],
    /// CCI550 clock and reset control.
    pub cci550_clock_reset: RWNoModify<u32>,
    _padding_0x304: [u32; 8],
    /// Wakeup requests issued by cluster 1 harts.
    pub cluster1_wakeup: [WO<u32>; 4],
    _padding_0x334: [u32; 21],
    /// ACLK clock and reset control.
    pub aclk_clock_reset: RWNoModify<u32>,
    /// CPU_CLUSTER0 clock and reset control.
    pub cpu_cluster0_clock_reset: RWNoModify<u32>,
    /// CPU_CLUSTER1 clock and reset control.
    pub cpu_cluster1_clock_reset: RWNoModify<u32>,
    _padding_0x394: [u32; 14],
    /// PCIE0 clock and reset control.
    pub pcie0_clock_reset: RWNoModify<u32>,
    _padding_0x3d0: [u32; 1],
    /// PCIE1 clock and reset control.
    pub pcie1_clock_reset: RWNoModify<u32>,
    _padding_0x3d8: [u32; 1],
    /// PCIE2 clock and reset control.
    pub pcie2_clock_reset: RWNoModify<u32>,
    _padding_0x3e0: [u32; 1],
    /// EMAC0 clock and reset control.
    pub emac0_clock_reset: RWNoModify<u32>,
    _padding_0x3e8: [u32; 1],
    /// EMAC1 clock and reset control.
    pub emac1_clock_reset: RWNoModify<u32>,
    _padding_0x3f0: [u32; 4],
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, qspi_clock_reset), 0x060);
        assert_eq!(offset_of!(RegisterBlock, cluster0_wakeup), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, cluster1_wakeup), 0x324);
        assert_eq!(offset_of!(RegisterBlock, jpeg_clock_reset), 0x20);
        assert_eq!(offset_of!(RegisterBlock, csi_clock_reset), 0x24);
        assert_eq!(offset_of!(RegisterBlock, isp_clock_reset), 0x38);
        assert_eq!(offset_of!(RegisterBlock, lcd1_clock_reset), 0x44);
        assert_eq!(offset_of!(RegisterBlock, lcd_spi_clock_reset), 0x48);
        assert_eq!(offset_of!(RegisterBlock, lcd2_clock_reset), 0x4c);
        assert_eq!(offset_of!(RegisterBlock, ccic_clock_reset), 0x50);
        assert_eq!(offset_of!(RegisterBlock, sdh0_clock_reset), 0x54);
        assert_eq!(offset_of!(RegisterBlock, sdh1_clock_reset), 0x58);
        assert_eq!(offset_of!(RegisterBlock, usb_clock_reset), 0x5c);
        assert_eq!(offset_of!(RegisterBlock, dma_clock_reset), 0x64);
        assert_eq!(offset_of!(RegisterBlock, aes_clock_reset), 0x68);
        assert_eq!(offset_of!(RegisterBlock, vpu_clock_reset), 0xa4);
        assert_eq!(offset_of!(RegisterBlock, gpu_clock_reset), 0xcc);
        assert_eq!(offset_of!(RegisterBlock, sdh2_clock_reset), 0xe0);
        assert_eq!(offset_of!(RegisterBlock, memory_clock_reset), 0xe8);
        assert_eq!(offset_of!(RegisterBlock, ap_clock_reset), 0x100);
        assert_eq!(offset_of!(RegisterBlock, em_clock_reset), 0x104);
        assert_eq!(offset_of!(RegisterBlock, audio_clock_reset), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, hdmi_clock_reset), 0x1b8);
        assert_eq!(offset_of!(RegisterBlock, cci550_clock_reset), 0x300);
        assert_eq!(offset_of!(RegisterBlock, aclk_clock_reset), 0x388);
        assert_eq!(offset_of!(RegisterBlock, cpu_cluster0_clock_reset), 0x38c);
        assert_eq!(offset_of!(RegisterBlock, cpu_cluster1_clock_reset), 0x390);
        assert_eq!(offset_of!(RegisterBlock, pcie0_clock_reset), 0x3cc);
        assert_eq!(offset_of!(RegisterBlock, pcie1_clock_reset), 0x3d4);
        assert_eq!(offset_of!(RegisterBlock, pcie2_clock_reset), 0x3dc);
        assert_eq!(offset_of!(RegisterBlock, emac0_clock_reset), 0x3e4);
        assert_eq!(offset_of!(RegisterBlock, emac1_clock_reset), 0x3ec);
        assert_eq!(size_of::<RegisterBlock>(), 0x400);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
