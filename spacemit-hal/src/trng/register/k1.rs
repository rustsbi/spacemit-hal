//! K1/M1 random generator and DMA registers.

use crate::register::{RC, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/d68a0caf7024a605f44ed818d41bab6786b6c999/en/key_stone/k1/k1_docs/k1_usermanual/9.Top_System.md#954-register-description

/// K1/M1 random generator and DMA registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 1],
    /// DMA byte count.
    pub byte_count: RW<u32>,
    _padding_0x008: [u32; 3],
    /// Unused DMA source address.
    pub source_address: RW<u32>,
    _padding_0x018: [u32; 3],
    /// DMA destination address.
    pub destination_address: RW<u32>,
    _padding_0x028: [u32; 3],
    /// Next DMA descriptor address.
    pub next_descriptor: RW<u32>,
    _padding_0x038: [u32; 3],
    /// DMA commands and FIFO clear.
    pub dma_control: RWNoModify<u32>,
    _padding_0x048: [u32; 11],
    /// Current DMA descriptor address.
    pub current_descriptor: RO<u32>,
    _padding_0x078: [u32; 3],
    /// DMA interrupt enables.
    pub interrupt_mask: RW<u32>,
    _padding_0x088: [u32; 3],
    /// Interrupt read-clear selection.
    pub read_clear_select: RW<u32>,
    _padding_0x098: [u32; 3],
    /// Interrupt status with selectable read-clear.
    pub interrupt_status: RWNoModify<u32>,
    _padding_0x0a8: [u32; 6],
    /// PRNG enable and valid state.
    pub prng_control: RW<u32>,
    /// PRNG result low word.
    pub prng_low: RO<u32>,
    /// PRNG result high byte.
    pub prng_high: RO<u32>,
    _padding_0x0cc: [u32; 1],
    /// TRNG clock, calibration and enable.
    pub trng_control: RWNoModify<u32>,
    /// TRNG analog configuration.
    pub trng_configuration: RW<u32>,
    /// TRNG result data.
    pub trng_data: RC<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, byte_count), 0x004);
        assert_eq!(offset_of!(RegisterBlock, source_address), 0x014);
        assert_eq!(offset_of!(RegisterBlock, destination_address), 0x024);
        assert_eq!(offset_of!(RegisterBlock, next_descriptor), 0x034);
        assert_eq!(offset_of!(RegisterBlock, dma_control), 0x044);
        assert_eq!(offset_of!(RegisterBlock, current_descriptor), 0x074);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mask), 0x084);
        assert_eq!(offset_of!(RegisterBlock, read_clear_select), 0x094);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x0a4);
        assert_eq!(offset_of!(RegisterBlock, prng_control), 0x0c0);
        assert_eq!(offset_of!(RegisterBlock, prng_low), 0x0c4);
        assert_eq!(offset_of!(RegisterBlock, prng_high), 0x0c8);
        assert_eq!(offset_of!(RegisterBlock, trng_control), 0x0d0);
        assert_eq!(offset_of!(RegisterBlock, trng_configuration), 0x0d4);
        assert_eq!(offset_of!(RegisterBlock, trng_data), 0x0d8);
        assert_eq!(size_of::<RegisterBlock>(), 0x0dc);
    }
}
