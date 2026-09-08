//! K1/M1 synchronous serial port registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/spi/spi-spacemit-k1.c
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/sound/soc/spacemit/k1_i2s.c
// Network and receive-cycle registers: K1 User Manual §16.2.4.

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
    /// Network mode and active time slots (SSNWCR).
    pub network_control: RW<u32>,
    /// Network busy and current time slot (SSNWS).
    pub network_status: RO<u32>,
    /// Receive-only control and cycle commands (SSRWT).
    pub receive_without_transmit: RWNoModify<u32>,
    /// Receive-only clock cycle match (SSRWTCC).
    pub receive_cycle_match: RW<u32>,
    /// Write one to capture the receive cycle count (SSRWTCV).
    pub receive_cycle_capture: RWNoModify<u32>,
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
        assert_eq!(offset_of!(RegisterBlock, network_control), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, network_status), 0x20);
        assert_eq!(offset_of!(RegisterBlock, receive_cycle_match), 0x28);
        assert_eq!(offset_of!(RegisterBlock, receive_cycle_capture), 0x2c);
        assert_eq!(size_of::<RegisterBlock>(), 0x30);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
