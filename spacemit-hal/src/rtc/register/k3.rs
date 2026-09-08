//! K3 real-time clock registers.

use crate::register::RWNoModify;
use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/rtc/rtc-sa1100.c
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/arch/riscv/boot/dts/spacemit/k3.dtsi

/// K3 real-time clock registers.
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
        assert_eq!(size_of::<RegisterBlock>(), 0x010);
    }
}
