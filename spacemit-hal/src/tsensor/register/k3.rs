//! K3 temperature sensor registers.

use volatile_register::{RO, RW, WO};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/thermal/k3-thermal.h
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/thermal/k3-thermal.c
// Only registers with verified accesses are exposed; undocumented status/calibration words are padding.

/// K3 temperature sensor registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Sensor configuration.
    pub configuration: RW<u32>,
    /// Sensor selection; write-only view used by Linux.
    pub sensor_select: WO<u32>,
    /// Interrupt enable; write-only view used by Linux.
    pub interrupt_enable: WO<u32>,
    _padding_0x00c: [u32; 1],
    /// Interrupt acknowledgement.
    pub interrupt_clear: WO<u32>,
    _padding_0x014: [u32; 1],
    /// Selected sensor temperature.
    pub temperature: RO<u32>,
    _padding_0x01c: [u32; 1],
    /// High and low temperature thresholds.
    pub threshold: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, configuration), 0x0);
        assert_eq!(offset_of!(RegisterBlock, sensor_select), 0x4);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x8);
        assert_eq!(offset_of!(RegisterBlock, interrupt_clear), 0x10);
        assert_eq!(offset_of!(RegisterBlock, temperature), 0x18);
        assert_eq!(offset_of!(RegisterBlock, threshold), 0x20);
        assert_eq!(size_of::<RegisterBlock>(), 0x24);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
