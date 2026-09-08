//! K3 hardware spinlock registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/16_peripherals/spinlock.md
// Use the register description's W1C semantics, not the contradictory final example.

/// K3 hardware spinlock registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Reading acquires an unlocked lock; writing zero releases it.
    pub lock: [RWNoModify<u32>; 32],
    _padding_0x080: [u32; 32],
    /// Hardware version (SPINLOCK_VER).
    pub version: RO<u32>,
    /// Implemented lock count (SPINLOCK_SSTATUS).
    pub system_status: RO<u32>,
    /// Non-destructive lock status bitmap (SPINLOCK_STATUS).
    pub status: RO<u32>,
    _padding_0x10c: [u32; 1],
    /// Unlock interrupt enables (SPINLOCK_IRQ_EN).
    pub interrupt_enable: RW<u32>,
    /// Write-one-to-clear unlock events (SPINLOCK_IRQ_STA).
    pub interrupt_status: RW1C<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, lock), 0x0);
        assert_eq!(offset_of!(RegisterBlock, version), 0x100);
        assert_eq!(offset_of!(RegisterBlock, system_status), 0x104);
        assert_eq!(offset_of!(RegisterBlock, status), 0x108);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x110);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x114);
        assert_eq!(size_of::<RegisterBlock>(), 0x118);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
