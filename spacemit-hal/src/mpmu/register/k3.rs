//! K3 MPMU register layout.

use super::ApplicationClockGate;
use volatile_register::{RO, RW};

// Offsets: include/soc/spacemit/k3-syscon.h; extent: k3.dtsi.
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k3-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k3.dtsi

/// K3 MPMU registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 2],
    /// fast clock control.
    pub fast_clock_control: RW<u32>,
    _padding_0x00c: [u32; 1],
    /// pll status.
    pub pll_status: RO<u32>,
    /// slow uart1 clock.
    pub slow_uart1_clock: RW<u32>,
    _padding_0x018: [u32; 10],
    /// i2s clock control0.
    pub i2s_clock_control0: RW<u32>,
    /// i2s clock control1.
    pub i2s_clock_control1: RW<u32>,
    _padding_0x048: [u32; 110],
    /// watchdog clock reset.
    pub watchdog_clock_reset: RW<u32>,
    _padding_0x204: [u32; 3],
    /// ipc clock control.
    pub ipc_clock_control: RW<u32>,
    _padding_0x214: [u32; 900],
    /// Application clock gates.
    pub application_clock_gate: RW<ApplicationClockGate>,
    _padding_0x1028: [u32; 10],
    /// apb clock source.
    pub apb_clock_source: RW<u32>,
    _padding_0x1054: [u32; 23],
    /// slow uart2 clock.
    pub slow_uart2_clock: RW<u32>,
    _padding_0x10b4: [u32; 19],
    /// i2s0 system clock.
    pub i2s0_system_clock: RW<u32>,
    /// i2s2 system clock.
    pub i2s2_system_clock: RW<u32>,
    /// i2s3 system clock.
    pub i2s3_system_clock: RW<u32>,
    /// i2s4 system clock.
    pub i2s4_system_clock: RW<u32>,
    /// i2s5 system clock.
    pub i2s5_system_clock: RW<u32>,
    /// i2s system clock control.
    pub i2s_system_clock_control: RW<u32>,
    _padding_0x1118: [u32; 15290],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};
    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, application_clock_gate), 0x1024);
        assert_eq!(offset_of!(RegisterBlock, fast_clock_control), 0x8);
        assert_eq!(offset_of!(RegisterBlock, pll_status), 0x10);
        assert_eq!(offset_of!(RegisterBlock, slow_uart1_clock), 0x14);
        assert_eq!(offset_of!(RegisterBlock, i2s_clock_control0), 0x40);
        assert_eq!(offset_of!(RegisterBlock, i2s_clock_control1), 0x44);
        assert_eq!(offset_of!(RegisterBlock, watchdog_clock_reset), 0x200);
        assert_eq!(offset_of!(RegisterBlock, ipc_clock_control), 0x210);
        assert_eq!(offset_of!(RegisterBlock, apb_clock_source), 0x1050);
        assert_eq!(offset_of!(RegisterBlock, slow_uart2_clock), 0x10b0);
        assert_eq!(offset_of!(RegisterBlock, i2s0_system_clock), 0x1100);
        assert_eq!(offset_of!(RegisterBlock, i2s2_system_clock), 0x1104);
        assert_eq!(offset_of!(RegisterBlock, i2s3_system_clock), 0x1108);
        assert_eq!(offset_of!(RegisterBlock, i2s4_system_clock), 0x110c);
        assert_eq!(offset_of!(RegisterBlock, i2s5_system_clock), 0x1110);
        assert_eq!(offset_of!(RegisterBlock, i2s_system_clock_control), 0x1114);
        assert_eq!(size_of::<RegisterBlock>(), 0x10000);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
