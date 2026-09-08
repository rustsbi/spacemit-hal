//! K3 DMA configuration registers.

use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/include/soc/spacemit/k3-syscon.h

/// K3 DMA configuration registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 129],
    /// system0 reset.
    pub system0_reset: RW<u32>,
    /// system1 reset.
    pub system1_reset: RW<u32>,
    /// DMA channel resets.
    pub channel_reset: [RW<u32>; 8],
    /// system reset.
    pub system_reset: RW<u32>,
    /// secure dma reset.
    pub secure_dma_reset: RW<u32>,
    /// clock enable.
    pub clock_enable: RW<u32>,
    /// secure dma clock enable.
    pub secure_dma_clock_enable: RW<u32>,
    _padding_0x23c: [u32; 2],
    /// cluster2 tcm clock.
    pub cluster2_tcm_clock: RW<u32>,
    /// cluster3 tcm clock.
    pub cluster3_tcm_clock: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, system0_reset), 0x204);
        assert_eq!(offset_of!(RegisterBlock, system1_reset), 0x208);
        assert_eq!(offset_of!(RegisterBlock, channel_reset), 0x20c);
        assert_eq!(offset_of!(RegisterBlock, system_reset), 0x22c);
        assert_eq!(offset_of!(RegisterBlock, secure_dma_reset), 0x230);
        assert_eq!(offset_of!(RegisterBlock, clock_enable), 0x234);
        assert_eq!(offset_of!(RegisterBlock, secure_dma_clock_enable), 0x238);
        assert_eq!(offset_of!(RegisterBlock, cluster2_tcm_clock), 0x244);
        assert_eq!(offset_of!(RegisterBlock, cluster3_tcm_clock), 0x248);
        assert_eq!(size_of::<RegisterBlock>(), 0x24c);
    }
}
