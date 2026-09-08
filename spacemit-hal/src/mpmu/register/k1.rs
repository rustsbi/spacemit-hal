//! K1 MPMU register layout.

use super::ApplicationClockGate;
use volatile_register::{RO, RW};

// Offsets: include/soc/spacemit/k1-syscon.h; extent: k1.dtsi.
// https://github.com/torvalds/linux/blob/master/include/soc/spacemit/k1-syscon.h
// https://github.com/torvalds/linux/blob/master/arch/riscv/boot/dts/spacemit/k1.dtsi

/// K1 MPMU registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 2],
    /// fast clock control.
    pub fast_clock_control: RW<u32>,
    _padding_0x00c: [u32; 1],
    /// PLL lock status.
    pub pll_status: RO<PllStatus>,
    /// Slow UART1 numerator and denominator.
    pub slow_uart1_clock: RW<u32>,
    _padding_0x018: [u32; 11],
    /// i2s clock control.
    pub i2s_clock_control: RW<u32>,
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
    /// Slow UART2 numerator and denominator.
    pub slow_uart2_clock: RW<u32>,
    _padding_0x10b4: [u32; 1018],
}

/// K1 PLL lock status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PllStatus(u32);

impl PllStatus {
    /// Creates a status value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Returns whether PLL1 is locked.
    #[inline]
    pub const fn is_pll1_locked(self) -> bool {
        self.0 & (1 << 27) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};
    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll_status), 0x10);
        assert_eq!(offset_of!(RegisterBlock, slow_uart1_clock), 0x14);
        assert_eq!(offset_of!(RegisterBlock, slow_uart2_clock), 0x10b0);
        assert_eq!(offset_of!(RegisterBlock, application_clock_gate), 0x1024);
        assert_eq!(offset_of!(RegisterBlock, fast_clock_control), 0x8);
        assert_eq!(offset_of!(RegisterBlock, i2s_clock_control), 0x44);
        assert_eq!(offset_of!(RegisterBlock, watchdog_clock_reset), 0x200);
        assert_eq!(offset_of!(RegisterBlock, ipc_clock_control), 0x210);
        assert_eq!(offset_of!(RegisterBlock, apb_clock_source), 0x1050);
        assert_eq!(size_of::<RegisterBlock>(), 0x209c);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }

    #[test]
    fn pll1_lock_bit() {
        assert!(PllStatus::from_bits(1 << 27).is_pll1_locked());
        assert!(!PllStatus::from_bits(!(1 << 27)).is_pll1_locked());
    }
}
