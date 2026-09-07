//! K1/M1 APMU register layout.

use super::QspiClockReset;
use volatile_register::{RW, WO};

// Offset: include/soc/spacemit/k1-syscon.h; window: k1.dtsi (0x400 bytes).
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k1-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k1.dtsi
// Wakeup offsets: vendor OpenSBI platform/generic/include/spacemit/k1x/k1x_evb.h.

/// K1/M1 APMU registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0x000: [u32; 24],
    /// QSPI clock/reset control with a self-clearing frequency-change request.
    pub qspi_clock_reset: RW<QspiClockReset>,
    _reserved_0x064: [u32; 50],
    /// Wakeup requests issued by cluster 0 harts.
    pub cluster0_wakeup: [WO<u32>; 4],
    _reserved_0x13c: [u32; 122],
    /// Wakeup requests issued by cluster 1 harts.
    pub cluster1_wakeup: [WO<u32>; 4],
    _reserved_0x334: [u32; 51],
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
        assert_eq!(size_of::<RegisterBlock>(), 0x400);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
