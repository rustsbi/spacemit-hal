//! K3 synchronous serial port registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/14_connectivity/spi.md

/// K3 synchronous serial port registers.
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
    /// Network mode configuration (SSNWCR).
    pub network_control: RW<u32>,
    /// Network mode status (SSNWS).
    pub network_status: RO<u32>,
    /// Receive-only control and cycle commands (SSRWT).
    pub receive_without_transmit: RWNoModify<u32>,
    /// Receive-only cycle count match (SSRWTCC).
    pub receive_cycle_control: RW<u32>,
    /// Receive-only cycle count (SSRWTCV).
    pub receive_cycle_value: RW<u32>,
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
        assert_eq!(offset_of!(RegisterBlock, network_control), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, network_status), 0x20);
        assert_eq!(offset_of!(RegisterBlock, receive_without_transmit), 0x24);
        assert_eq!(offset_of!(RegisterBlock, receive_cycle_control), 0x28);
        assert_eq!(offset_of!(RegisterBlock, receive_cycle_value), 0x2c);
        assert_eq!(size_of::<RegisterBlock>(), 0x30);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
