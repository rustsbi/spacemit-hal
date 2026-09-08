//! K1 secure APB clock and reset registers.

use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/include/soc/spacemit/k1-syscon.h

/// K1 secure APB clock and reset registers.
#[repr(C)]
pub struct RegisterBlock {
    /// UART1 clock and reset.
    pub uart1_clock_reset: RW<u32>,
    /// SSP2 clock and reset.
    pub ssp2_clock_reset: RW<u32>,
    /// I2C3 clock and reset.
    pub twsi3_clock_reset: RW<u32>,
    /// Secure RTC clock and reset.
    pub rtc_clock_reset: RW<u32>,
    /// Secure timer clock and reset.
    pub timer_clock_reset: RW<u32>,
    /// Keypad clock and reset.
    pub keypad_clock_reset: RW<u32>,
    _padding_0x018: [u32; 1],
    /// Secure GPIO clock and reset.
    pub gpio_clock_reset: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, uart1_clock_reset), 0x000);
        assert_eq!(offset_of!(RegisterBlock, ssp2_clock_reset), 0x004);
        assert_eq!(offset_of!(RegisterBlock, twsi3_clock_reset), 0x008);
        assert_eq!(offset_of!(RegisterBlock, rtc_clock_reset), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, timer_clock_reset), 0x010);
        assert_eq!(offset_of!(RegisterBlock, keypad_clock_reset), 0x014);
        assert_eq!(offset_of!(RegisterBlock, gpio_clock_reset), 0x01c);
        assert_eq!(size_of::<RegisterBlock>(), 0x020);
    }
}
