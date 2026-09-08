//! K3 half-duplex audio interface registers.

use crate::register::RWNoModify;
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/13_audio.md
// Section 13.3.2: RX_DATA at 0x000 and TX_DATA at 0x080 are DMA-only, not CPU access paths.

/// K3 half-duplex audio interface registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 1],
    /// Receive channel identification.
    pub receive_id: RO<u32>,
    /// Receive format.
    pub receive_control: RW<u32>,
    /// Receive port configuration, reset and flush.
    pub receive_port_control: RWNoModify<u32>,
    /// Receive FIFO upper threshold.
    pub receive_fifo_upper_limit: RW<u32>,
    /// Receive interrupt mask and status.
    pub receive_interrupt: RWNoModify<u32>,
    _padding_0x018: [u32; 27],
    /// Transmit channel identification.
    pub transmit_id: RO<u32>,
    /// Transmit format.
    pub transmit_control: RW<u32>,
    /// Transmit port configuration, reset and flush.
    pub transmit_port_control: RWNoModify<u32>,
    /// Transmit FIFO lower threshold.
    pub transmit_fifo_lower_limit: RW<u32>,
    /// Transmit interrupt mask and status.
    pub transmit_interrupt: RWNoModify<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, receive_id), 0x4);
        assert_eq!(offset_of!(RegisterBlock, receive_control), 0x8);
        assert_eq!(offset_of!(RegisterBlock, receive_port_control), 0xc);
        assert_eq!(offset_of!(RegisterBlock, receive_fifo_upper_limit), 0x10);
        assert_eq!(offset_of!(RegisterBlock, receive_interrupt), 0x14);
        assert_eq!(offset_of!(RegisterBlock, transmit_id), 0x84);
        assert_eq!(offset_of!(RegisterBlock, transmit_control), 0x88);
        assert_eq!(offset_of!(RegisterBlock, transmit_port_control), 0x8c);
        assert_eq!(offset_of!(RegisterBlock, transmit_fifo_lower_limit), 0x90);
        assert_eq!(offset_of!(RegisterBlock, transmit_interrupt), 0x94);
        assert_eq!(size_of::<RegisterBlock>(), 0x98);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
