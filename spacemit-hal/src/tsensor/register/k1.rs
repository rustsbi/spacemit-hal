//! K1/M1 temperature sensor registers.

use crate::register::RWNoModify;
use volatile_register::{RO, RW};

// https://github.com/torvalds/linux/blob/master/drivers/thermal/spacemit/k1_tsensor.c

/// K1/M1 temperature sensor registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Sampling and analog control.
    pub control: RW<u32>,
    _padding_0x004: [u32; 1],
    /// Sensor enables.
    pub enable: RW<u32>,
    /// Sampling and filter timing.
    pub timing: RW<u32>,
    /// Interrupt acknowledgement.
    pub interrupt_clear: RWNoModify<u32>,
    /// Interrupt masks and global enable.
    pub interrupt_enable: RW<u32>,
    /// Interrupt state.
    pub interrupt_status: RO<u32>,
    _padding_0x01c: [u32; 1],
    /// Packed samples for five sensors.
    pub temperature: [RO<u32>; 3],
    _padding_0x02c: [u32; 5],
    /// Per-sensor low and high thresholds.
    pub threshold: [RW<u32>; 5],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x000);
        assert_eq!(offset_of!(RegisterBlock, enable), 0x008);
        assert_eq!(offset_of!(RegisterBlock, timing), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, interrupt_clear), 0x010);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x014);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x018);
        assert_eq!(offset_of!(RegisterBlock, temperature), 0x020);
        assert_eq!(offset_of!(RegisterBlock, threshold), 0x040);
        assert_eq!(size_of::<RegisterBlock>(), 0x054);
    }
}
