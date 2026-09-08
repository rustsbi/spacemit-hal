//! K1/M1 real-time CPU system registers.

use volatile_register::RW;

// https://github.com/spacemit-com/docs-chip/blob/d68a0caf7024a605f44ed818d41bab6786b6c999/en/key_stone/k1/k1_docs/k1_usermanual/14.RCPU_Subsystem.md

/// K1/M1 real-time CPU system registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 10],
    /// ssp0 clock reset.
    pub ssp0_clock_reset: RW<u32>,
    _padding_0x02c: [u32; 1],
    /// i2c0 clock reset.
    pub i2c0_clock_reset: RW<u32>,
    _padding_0x034: [u32; 2],
    /// uart1 clock reset.
    pub uart1_clock_reset: RW<u32>,
    _padding_0x040: [u32; 2],
    /// can clock reset.
    pub can_clock_reset: RW<u32>,
    /// ir clock reset.
    pub ir_clock_reset: RW<u32>,
    _padding_0x050: [u32; 28],
    /// ddr remap base.
    pub ddr_remap_base: RW<u32>,
    _padding_0x0c4: [u32; 5],
    /// uart0 clock reset.
    pub uart0_clock_reset: RW<u32>,
}

/// Pwm Clock Registers.
#[repr(C)]
pub struct PwmClockRegisters {
    /// Per-instance clock and reset controls.
    pub clock_reset: [RW<u32>; 10],
}

/// Real-time audio clock registers.
#[repr(C)]
pub struct AudioClockRegisters {
    _padding_0x000: [u32; 5],
    /// codec clock reset.
    pub codec_clock_reset: RW<u32>,
    _padding_0x018: [u32; 1],
    /// dfe clock reset.
    pub dfe_clock_reset: RW<u32>,
    _padding_0x020: [u32; 8],
    /// i2s1 clock reset.
    pub i2s1_clock_reset: RW<u32>,
    /// hdmi clock reset.
    pub hdmi_clock_reset: RW<u32>,
    _padding_0x048: [u32; 6],
    /// i2s0 clock reset.
    pub i2s0_clock_reset: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(PwmClockRegisters, clock_reset), 0x000);
        assert_eq!(size_of::<PwmClockRegisters>(), 0x028);
        assert_eq!(offset_of!(AudioClockRegisters, codec_clock_reset), 0x014);
        assert_eq!(offset_of!(AudioClockRegisters, dfe_clock_reset), 0x01c);
        assert_eq!(offset_of!(AudioClockRegisters, i2s1_clock_reset), 0x040);
        assert_eq!(offset_of!(AudioClockRegisters, hdmi_clock_reset), 0x044);
        assert_eq!(offset_of!(AudioClockRegisters, i2s0_clock_reset), 0x060);
        assert_eq!(size_of::<AudioClockRegisters>(), 0x064);
        assert_eq!(offset_of!(RegisterBlock, ssp0_clock_reset), 0x028);
        assert_eq!(offset_of!(RegisterBlock, i2c0_clock_reset), 0x030);
        assert_eq!(offset_of!(RegisterBlock, uart1_clock_reset), 0x03c);
        assert_eq!(offset_of!(RegisterBlock, can_clock_reset), 0x048);
        assert_eq!(offset_of!(RegisterBlock, ir_clock_reset), 0x04c);
        assert_eq!(offset_of!(RegisterBlock, ddr_remap_base), 0x0c0);
        assert_eq!(offset_of!(RegisterBlock, uart0_clock_reset), 0x0d8);
        assert_eq!(size_of::<RegisterBlock>(), 0x0dc);
    }
}
