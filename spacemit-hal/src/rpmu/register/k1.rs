//! K1/M1 real-time power-management registers.

use crate::register::RWNoModify;
use volatile_register::RW;

// https://github.com/spacemit-com/docs-chip/blob/d68a0caf7024a605f44ed818d41bab6786b6c999/en/key_stone/k1/k1_docs/k1_usermanual/14.RCPU_Subsystem.md
// Base is 0xc088c000 per the address map and CCU driver, not the chapter's USB3 base.

/// K1/M1 real-time power-management registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 6],
    /// power vote.
    pub power_vote: RW<u32>,
    _padding_0x01c: [u32; 1],
    /// main power vote.
    pub main_power_vote: RW<u32>,
    _padding_0x024: [u32; 1],
    /// wakeup enable.
    pub wakeup_enable: RW<u32>,
    /// peripheral clock reset.
    pub peripheral_clock_reset: RW<u32>,
    /// mcu execution.
    pub mcu_execution: RW<u32>,
    _padding_0x034: [u32; 1],
    /// bus clock divider.
    pub bus_clock_divider: RW<u32>,
    /// gpio output.
    pub gpio_output: RW<u32>,
    _padding_0x040: [u32; 40],
    /// Audio control and interrupt acknowledgements.
    pub audio_control: RWNoModify<u32>,
    /// Audio overcurrent protection.
    pub audio_control2: RW<u32>,
    /// Audio detection clock dividers.
    pub detection_clock_divider: RW<u32>,
    _padding_0x0ec: [u32; 1],
    /// Audio interrupt masks.
    pub audio_interrupt_mask: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, power_vote), 0x018);
        assert_eq!(offset_of!(RegisterBlock, main_power_vote), 0x020);
        assert_eq!(offset_of!(RegisterBlock, wakeup_enable), 0x028);
        assert_eq!(offset_of!(RegisterBlock, peripheral_clock_reset), 0x02c);
        assert_eq!(offset_of!(RegisterBlock, mcu_execution), 0x030);
        assert_eq!(offset_of!(RegisterBlock, bus_clock_divider), 0x038);
        assert_eq!(offset_of!(RegisterBlock, gpio_output), 0x03c);
        assert_eq!(offset_of!(RegisterBlock, audio_control), 0xe0);
        assert_eq!(offset_of!(RegisterBlock, audio_control2), 0xe4);
        assert_eq!(offset_of!(RegisterBlock, detection_clock_divider), 0xe8);
        assert_eq!(offset_of!(RegisterBlock, audio_interrupt_mask), 0xf0);
        assert_eq!(size_of::<RegisterBlock>(), 0xf4);
    }
}
