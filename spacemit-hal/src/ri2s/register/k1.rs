//! K1/M1 half-duplex audio interface registers.

use crate::register::RWNoModify;
use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/sound/soc/spacemit/spacemit-snd-sspa.h
// DMA data ports and undocumented FIFO/status accesses are not exposed.

/// K1/M1 half-duplex audio interface registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 2],
    /// receive control.
    pub receive_control: RW<u32>,
    /// Receive configuration, reset and flush.
    pub receive_port_control: RWNoModify<u32>,
    _padding_0x010: [u32; 30],
    /// transmit control.
    pub transmit_control: RW<u32>,
    /// Transmit configuration, reset and flush.
    pub transmit_port_control: RWNoModify<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, receive_control), 0x008);
        assert_eq!(offset_of!(RegisterBlock, receive_port_control), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, transmit_control), 0x088);
        assert_eq!(offset_of!(RegisterBlock, transmit_port_control), 0x08c);
        assert_eq!(size_of::<RegisterBlock>(), 0x090);
    }
}
