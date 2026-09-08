//! K3 audio DMA registers.

use crate::register::RWNoModify;
use volatile_register::RW;

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/13_audio.md
// Section 13.5.2: two channels per ADMA instance.

/// K3 audio DMA registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Remaining transfer bytes.
    pub byte_count: [RW<u32>; 2],
    _padding_0x008: [u32; 2],
    /// Source addresses.
    pub source_address: [RW<u32>; 2],
    _padding_0x018: [u32; 2],
    /// Destination addresses.
    pub destination_address: [RW<u32>; 2],
    _padding_0x028: [u32; 2],
    /// Next descriptor addresses.
    pub next_descriptor: [RW<u32>; 2],
    _padding_0x038: [u32; 2],
    /// Channel configuration, fetch and abort.
    pub control: [RWNoModify<u32>; 2],
    _padding_0x048: [u32; 6],
    /// Channel priority.
    pub priority: RW<u32>,
    /// Channel access filter.
    pub id_filter: RW<u32>,
    _padding_0x068: [u32; 2],
    /// Current descriptor addresses.
    pub current_descriptor: [RW<u32>; 2],
    _padding_0x078: [u32; 2],
    /// Channel interrupt masks.
    pub interrupt_mask: [RW<u32>; 2],
    _padding_0x088: [u32; 2],
    /// Interrupt read-clear selection.
    pub read_clear_select: [RW<u32>; 2],
    _padding_0x098: [u32; 2],
    /// Interrupt status; reads may clear selected flags.
    pub interrupt_status: [RWNoModify<u32>; 2],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, byte_count), 0x0);
        assert_eq!(offset_of!(RegisterBlock, source_address), 0x10);
        assert_eq!(offset_of!(RegisterBlock, destination_address), 0x20);
        assert_eq!(offset_of!(RegisterBlock, next_descriptor), 0x30);
        assert_eq!(offset_of!(RegisterBlock, control), 0x40);
        assert_eq!(offset_of!(RegisterBlock, priority), 0x60);
        assert_eq!(offset_of!(RegisterBlock, id_filter), 0x64);
        assert_eq!(offset_of!(RegisterBlock, current_descriptor), 0x70);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mask), 0x80);
        assert_eq!(offset_of!(RegisterBlock, read_clear_select), 0x90);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0xa0);
        assert_eq!(size_of::<RegisterBlock>(), 0xa8);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
