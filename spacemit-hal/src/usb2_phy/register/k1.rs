//! K1/M1 USB2 PHY registers.

use crate::register::RWNoModify;
use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/phy/spacemit/phy-k1-usb2.c

/// K1/M1 USB2 PHY registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 1],
    /// Clock, reset and PLL ready state.
    pub reset_mode_control: RW<u32>,
    _padding_0x008: [u32; 2],
    /// Host disconnect auto-clear control.
    pub transmit_host_control: RW<u32>,
    _padding_0x014: [u32; 8],
    /// High-speed transmit clock control.
    pub hstxp_control: RW<u32>,
    _padding_0x038: [u32; 2],
    /// Host disconnect acknowledgement.
    pub disconnect_clear: RWNoModify<u32>,
    _padding_0x044: [u32; 21],
    /// PLL reference selection and divider.
    pub pll_divider: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, reset_mode_control), 0x004);
        assert_eq!(offset_of!(RegisterBlock, transmit_host_control), 0x010);
        assert_eq!(offset_of!(RegisterBlock, hstxp_control), 0x034);
        assert_eq!(offset_of!(RegisterBlock, disconnect_clear), 0x040);
        assert_eq!(offset_of!(RegisterBlock, pll_divider), 0x098);
        assert_eq!(size_of::<RegisterBlock>(), 0x09c);
    }
}
