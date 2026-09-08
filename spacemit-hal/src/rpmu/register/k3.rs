//! K3 real-time power-management registers.

use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/include/soc/spacemit/k3-syscon.h

/// K3 real-time power-management registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 11],
    /// peripheral clock reset.
    pub peripheral_clock_reset: RW<u32>,
    _padding_0x030: [u32; 7],
    /// timer1 clock reset.
    pub timer1_clock_reset: RW<u32>,
    _padding_0x050: [u32; 8],
    /// timer2 clock reset.
    pub timer2_clock_reset: RW<u32>,
    /// gpio edge clock reset.
    pub gpio_edge_clock_reset: RW<u32>,
    /// timer3 clock reset.
    pub timer3_clock_reset: RW<u32>,
    /// timer4 clock reset.
    pub timer4_clock_reset: RW<u32>,
    _padding_0x080: [u32; 16],
    /// bus clock.
    pub bus_clock: RW<u32>,
    /// core0 clock.
    pub core0_clock: RW<u32>,
    /// core1 clock.
    pub core1_clock: RW<u32>,
    /// core0 reset.
    pub core0_reset: RW<u32>,
    /// core1 reset.
    pub core1_reset: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, peripheral_clock_reset), 0x02c);
        assert_eq!(offset_of!(RegisterBlock, timer1_clock_reset), 0x04c);
        assert_eq!(offset_of!(RegisterBlock, timer2_clock_reset), 0x070);
        assert_eq!(offset_of!(RegisterBlock, gpio_edge_clock_reset), 0x074);
        assert_eq!(offset_of!(RegisterBlock, timer3_clock_reset), 0x078);
        assert_eq!(offset_of!(RegisterBlock, timer4_clock_reset), 0x07c);
        assert_eq!(offset_of!(RegisterBlock, bus_clock), 0x0c0);
        assert_eq!(offset_of!(RegisterBlock, core0_clock), 0x0c4);
        assert_eq!(offset_of!(RegisterBlock, core1_clock), 0x0c8);
        assert_eq!(offset_of!(RegisterBlock, core0_reset), 0x0cc);
        assert_eq!(offset_of!(RegisterBlock, core1_reset), 0x0d0);
        assert_eq!(size_of::<RegisterBlock>(), 0x0d4);
    }
}
