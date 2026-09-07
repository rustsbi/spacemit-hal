//! K3 APMU register layout.

use super::QspiClockReset;
use volatile_register::{RW, WO};

// Offset: include/soc/spacemit/k3-syscon.h; window: k3.dtsi (0x400 bytes).
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k3-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi
// Wakeup offsets: vendor OpenSBI platform/generic/include/spacemit/k3/k3.h.

/// K3 APMU registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0x000: [u32; 24],
    /// QSPI clock/reset control with a self-clearing frequency-change request.
    pub qspi_clock_reset: RW<QspiClockReset>,
    _reserved_0x064: [u32; 50],
    /// Cluster 0 hart wakeup requests.
    pub cluster0_wakeup: [WO<u32>; 4],
    _reserved_0x13c: [u32; 60],
    /// Cluster 3 hart wakeup requests.
    pub cluster3_wakeup: [WO<u32>; 4],
    _reserved_0x23c: [u32; 58],
    /// Cluster 1 hart wakeup requests.
    pub cluster1_wakeup: [WO<u32>; 4],
    _reserved_0x334: [u32; 11],
    /// Cluster 2 hart wakeup requests.
    pub cluster2_wakeup: [WO<u32>; 4],
    _reserved_0x370: [u32; 36],
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
        assert_eq!(offset_of!(RegisterBlock, cluster2_wakeup), 0x360);
        assert_eq!(offset_of!(RegisterBlock, cluster3_wakeup), 0x22c);
        assert_eq!(size_of::<RegisterBlock>(), 0x400);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
