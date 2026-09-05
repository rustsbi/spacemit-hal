//! K3 MPMU register layout.

use super::ApplicationClockGate;
use volatile_register::RW;

// Offsets: include/soc/spacemit/k3-syscon.h; extent: k3.dtsi.
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k3-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi

/// K3 MPMU registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0x000: [u32; 1033],
    /// Application clock gates.
    pub application_clock_gate: RW<ApplicationClockGate>,
    _reserved_0x1028: [u32; 15350],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};
    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, application_clock_gate), 0x1024);
        assert_eq!(size_of::<RegisterBlock>(), 0x10000);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
