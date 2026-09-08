//! K1/M1 audio DMA channel registers.

use crate::register::RWNoModify;
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/drivers/dma/adma-spacemit.c
// This view exposes the first channel used by the K1 HDMI audio driver.

/// K1/M1 audio DMA channel registers.
#[repr(C)]
pub struct RegisterBlock {
    /// byte count.
    pub byte_count: RW<u32>,
    _padding_0x004: [u32; 3],
    /// source address.
    pub source_address: RW<u32>,
    _padding_0x014: [u32; 3],
    /// destination address.
    pub destination_address: RW<u32>,
    _padding_0x024: [u32; 3],
    /// next descriptor.
    pub next_descriptor: RW<u32>,
    _padding_0x034: [u32; 3],
    /// Channel configuration, fetch and abort.
    pub control: RWNoModify<u32>,
    _padding_0x044: [u32; 11],
    /// current descriptor.
    pub current_descriptor: RO<u32>,
    _padding_0x074: [u32; 3],
    /// interrupt mask.
    pub interrupt_mask: RW<u32>,
    _padding_0x084: [u32; 7],
    /// Interrupt status with hardware acknowledgement side effects.
    pub interrupt_status: RWNoModify<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, byte_count), 0x000);
        assert_eq!(offset_of!(RegisterBlock, source_address), 0x010);
        assert_eq!(offset_of!(RegisterBlock, destination_address), 0x020);
        assert_eq!(offset_of!(RegisterBlock, next_descriptor), 0x030);
        assert_eq!(offset_of!(RegisterBlock, control), 0x040);
        assert_eq!(offset_of!(RegisterBlock, current_descriptor), 0x070);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mask), 0x080);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x0a0);
        assert_eq!(size_of::<RegisterBlock>(), 0x0a4);
    }
}
