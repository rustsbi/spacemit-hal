//! K3 APMU register layout.

use super::QspiClockReset;
use volatile_register::RW;

// Offset: include/soc/spacemit/k3-syscon.h; window: k3.dtsi (0x400 bytes).
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k3-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi

/// K3 APMU registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0x000: [u32; 24],
    /// QSPI clock/reset control with a self-clearing frequency-change request.
    pub qspi_clock_reset: RW<QspiClockReset>,
    _reserved_0x064: [u32; 231],
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, qspi_clock_reset), 0x060);
        assert_eq!(size_of::<RegisterBlock>(), 0x400);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
