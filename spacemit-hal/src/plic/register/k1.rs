//! K1/M1 PLIC registers.

use crate::register::RWNoModify;
use volatile_register::{RO, RW};

// https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc
// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/arch/riscv/boot/dts/spacemit/k1-x.dtsi
// K1 exposes sources 1..159 and sixteen alternating M/S hart contexts.

/// K1/M1 PLIC registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 1],
    /// Priorities for source IDs 1 through 159.
    pub priority: [RW<u32>; 159],
    _padding_0x280: [u32; 864],
    /// Pending bits indexed by source ID.
    pub pending: [RO<u32>; 5],
    _padding_0x1014: [u32; 1019],
    /// Interrupt enables per M/S context.
    pub enable: [Enable; 16],
    _padding_0x2800: [u32; 521728],
    /// Threshold and claim/complete per M/S context.
    pub context: [Context; 16],
}

/// PLIC context enables.
#[repr(C)]
pub struct Enable {
    /// Source enables indexed by source ID.
    pub bits: [RW<u32>; 5],
    _padding_0x014: [u32; 27],
}

/// PLIC context control.
#[repr(C)]
pub struct Context {
    /// Priority threshold.
    pub threshold: RW<u32>,
    /// Reading claims an interrupt; writing completes it.
    pub claim_complete: RWNoModify<u32>,
    _padding_0x008: [u32; 1022],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, priority), 0x004);
        assert_eq!(offset_of!(RegisterBlock, pending), 0x1000);
        assert_eq!(offset_of!(RegisterBlock, enable), 0x2000);
        assert_eq!(offset_of!(RegisterBlock, context), 0x200000);
        assert_eq!(size_of::<Enable>(), 0x80);
        assert_eq!(size_of::<Context>(), 0x1000);
        assert_eq!(offset_of!(Context, claim_complete), 4);
        assert_eq!(size_of::<RegisterBlock>(), 0x210000);
    }
}
