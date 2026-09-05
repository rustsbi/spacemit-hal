//! K3 APBC register layout.

use super::{TwsiClockReset, UartClockReset};
use volatile_register::RW;

// Offsets: https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k3-syscon.h
// UART1 and the real-time-domain UARTs use other clock/reset controllers.
// TWSI3 is in APBC2, and RCPU I2C uses I2CCTRL; neither belongs to this block.
// Unlike K1, the published K3 map has no TWSI7 entry at 0x68.
// Unmodeled registers fill the 0x1000-byte APBC window from Linux k3.dtsi.

/// K3 APBC registers.
#[repr(C)]
pub struct RegisterBlock {
    /// UART0 clock and reset control.
    pub uart0_clock_reset: RW<UartClockReset>,
    /// UART2 clock and reset control.
    pub uart2_clock_reset: RW<UartClockReset>,
    _reserved_0x008: [u32; 6],
    /// TWSI8 (I²C8) clock and reset control.
    pub twsi8_clock_reset: RW<TwsiClockReset>,
    /// UART3 clock and reset control.
    pub uart3_clock_reset: RW<UartClockReset>,
    _reserved_0x028: [u32; 1],
    /// TWSI0 (I²C0) clock and reset control.
    pub twsi0_clock_reset: RW<TwsiClockReset>,
    /// TWSI1 (I²C1) clock and reset control.
    pub twsi1_clock_reset: RW<TwsiClockReset>,
    _reserved_0x034: [u32; 1],
    /// TWSI2 (I²C2) clock and reset control.
    pub twsi2_clock_reset: RW<TwsiClockReset>,
    _reserved_0x03c: [u32; 1],
    /// TWSI4 (I²C4) clock and reset control.
    pub twsi4_clock_reset: RW<TwsiClockReset>,
    _reserved_0x044: [u32; 2],
    /// TWSI5 (I²C5) clock and reset control.
    pub twsi5_clock_reset: RW<TwsiClockReset>,
    _reserved_0x050: [u32; 4],
    /// TWSI6 (I²C6) clock and reset control.
    pub twsi6_clock_reset: RW<TwsiClockReset>,
    _reserved_0x064: [u32; 3],
    /// UART4 clock and reset control.
    pub uart4_clock_reset: RW<UartClockReset>,
    /// UART5 clock and reset control.
    pub uart5_clock_reset: RW<UartClockReset>,
    /// UART6 clock and reset control.
    pub uart6_clock_reset: RW<UartClockReset>,
    _reserved_0x07c: [u32; 6],
    /// UART7 clock and reset control.
    pub uart7_clock_reset: RW<UartClockReset>,
    /// UART8 clock and reset control.
    pub uart8_clock_reset: RW<UartClockReset>,
    /// UART9 clock and reset control.
    pub uart9_clock_reset: RW<UartClockReset>,
    _reserved_0x0a0: [u32; 45],
    /// UART10 clock and reset control.
    pub uart10_clock_reset: RW<UartClockReset>,
    _reserved_0x158: [u32; 938],
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, uart0_clock_reset), 0x000);
        assert_eq!(offset_of!(RegisterBlock, uart2_clock_reset), 0x004);
        assert_eq!(offset_of!(RegisterBlock, twsi8_clock_reset), 0x020);
        assert_eq!(offset_of!(RegisterBlock, uart3_clock_reset), 0x024);
        assert_eq!(offset_of!(RegisterBlock, twsi0_clock_reset), 0x02c);
        assert_eq!(offset_of!(RegisterBlock, twsi1_clock_reset), 0x030);
        assert_eq!(offset_of!(RegisterBlock, twsi2_clock_reset), 0x038);
        assert_eq!(offset_of!(RegisterBlock, twsi4_clock_reset), 0x040);
        assert_eq!(offset_of!(RegisterBlock, twsi5_clock_reset), 0x04c);
        assert_eq!(offset_of!(RegisterBlock, twsi6_clock_reset), 0x060);
        assert_eq!(offset_of!(RegisterBlock, uart4_clock_reset), 0x070);
        assert_eq!(offset_of!(RegisterBlock, uart5_clock_reset), 0x074);
        assert_eq!(offset_of!(RegisterBlock, uart6_clock_reset), 0x078);
        assert_eq!(offset_of!(RegisterBlock, uart7_clock_reset), 0x094);
        assert_eq!(offset_of!(RegisterBlock, uart8_clock_reset), 0x098);
        assert_eq!(offset_of!(RegisterBlock, uart9_clock_reset), 0x09c);
        assert_eq!(offset_of!(RegisterBlock, uart10_clock_reset), 0x154);
        assert_eq!(size_of::<RegisterBlock>(), 0x1000);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
