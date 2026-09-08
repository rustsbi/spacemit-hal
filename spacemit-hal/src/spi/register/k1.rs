//! K1/M1 synchronous serial port registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/spi/spi-spacemit-k1.c
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/sound/soc/spacemit/k1_i2s.c
// Only the K1 offsets used by these drivers are exposed.

/// K1/M1 synchronous serial port registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Port control (SSCR).
    pub control: RW<u32>,
    /// FIFO configuration (SSFCR).
    pub fifo_control: RW<u32>,
    /// Interrupt enables (SSINTEN).
    pub interrupt_enable: RW<u32>,
    /// Receive timeout (SSTO).
    pub timeout: RW<u32>,
    /// Transmit push / receive pop port (SSDATR).
    pub data: RWNoModify<u32>,
    /// Status and write-one-to-clear events (SSSR).
    pub status: RW1C<u32>,
    /// Programmable serial protocol (SSPSP).
    pub protocol: RW<u32>,
    _padding_0x01c: [u32; 2],
    /// Receive-only control and cycle commands (SSRWT).
    pub receive_without_transmit: RWNoModify<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x0);
        assert_eq!(offset_of!(RegisterBlock, fifo_control), 0x4);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x8);
        assert_eq!(offset_of!(RegisterBlock, timeout), 0xc);
        assert_eq!(offset_of!(RegisterBlock, data), 0x10);
        assert_eq!(offset_of!(RegisterBlock, status), 0x14);
        assert_eq!(offset_of!(RegisterBlock, protocol), 0x18);
        assert_eq!(offset_of!(RegisterBlock, receive_without_transmit), 0x24);
        assert_eq!(size_of::<RegisterBlock>(), 0x28);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
