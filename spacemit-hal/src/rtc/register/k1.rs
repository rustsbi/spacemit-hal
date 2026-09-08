//! K1/M1 real-time clock registers.

use crate::register::RWNoModify;
use volatile_register::RW;

// https://github.com/spacemit-com/docs-chip/blob/d68a0caf7024a605f44ed818d41bab6786b6c999/en/key_stone/k1/k1_docs/k1_usermanual/9.Top_System.md#974-register-description

/// K1/M1 real-time clock registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Seconds counter.
    pub counter: RW<u32>,
    /// Alarm time.
    pub alarm: RW<u32>,
    /// Interrupt enables and write-one-to-clear flags.
    pub status: RWNoModify<u32>,
    /// Oscillator trim and write lock.
    pub trim: RW<u32>,
    /// External alarm control.
    pub control: RW<u32>,
    /// Battery-backed scratch registers.
    pub backup: [RW<u32>; 5],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, counter), 0x000);
        assert_eq!(offset_of!(RegisterBlock, alarm), 0x004);
        assert_eq!(offset_of!(RegisterBlock, status), 0x008);
        assert_eq!(offset_of!(RegisterBlock, trim), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, control), 0x010);
        assert_eq!(offset_of!(RegisterBlock, backup), 0x014);
        assert_eq!(size_of::<RegisterBlock>(), 0x028);
    }
}
