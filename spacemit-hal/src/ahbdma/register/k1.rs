//! K1/M1 real-time AHB DMA registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::RO;
use volatile_register::RW;

// https://github.com/spacemit-com/docs-chip/blob/d68a0caf7024a605f44ed818d41bab6786b6c999/en/key_stone/k1/k1_docs/k1_usermanual/14.RCPU_Subsystem.md

/// K1/M1 real-time AHB DMA registers.
#[repr(C)]
pub struct RegisterBlock {
    /// DMA enable and soft reset.
    pub control: RWNoModify<u32>,
    /// Channel interrupt acknowledgements.
    pub interrupt_status: RW1C<u32>,
    /// Channel interrupt masks.
    pub interrupt_mask: RW<u32>,
    _padding_0x00c: [u32; 29],
    /// DMA channels.
    pub channel: [Channel; 16],
}

/// AHB DMA channel registers.
#[repr(C)]
pub struct Channel {
    /// source address.
    pub source_address: RW<u32>,
    /// destination address.
    pub destination_address: RW<u32>,
    /// byte count.
    pub byte_count: RW<u32>,
    /// control.
    pub control: RW<u32>,
    /// request select.
    pub request_select: RW<u32>,
    /// burst length.
    pub burst_length: RW<u32>,
    _padding_0x018: [u32; 1],
    /// transferred bytes.
    pub transferred_bytes: RO<u32>,
    /// burst type.
    pub burst_type: RW<u32>,
    _padding_0x024: [u32; 7],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x000);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x004);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mask), 0x008);
        assert_eq!(offset_of!(RegisterBlock, channel), 0x080);
        assert_eq!(size_of::<Channel>(), 0x40);
        assert_eq!(offset_of!(Channel, transferred_bytes), 0x1c);
        assert_eq!(size_of::<RegisterBlock>(), 0x480);
    }
}
