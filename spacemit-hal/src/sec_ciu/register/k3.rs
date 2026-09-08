//! K3 secure central-interface registers.

use volatile_register::RW;

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/15_security.md
// SEC_CIU at 0xf0580000; do not overlay the non-secure CIU.

/// K3 secure central-interface registers.
#[repr(C)]
pub struct RegisterBlock {
    /// DMA and peripheral security selection.
    pub dma_secure_control: RW<u32>,
    _padding_0x004: [u32; 1],
    /// Bus master security selection.
    pub master_secure_control: RW<u32>,
    _padding_0x00c: [u32; 1],
    /// CPU and debug security identifiers.
    pub nsaid_control0: RW<u32>,
    /// Multimedia security identifiers.
    pub nsaid_control1: RW<u32>,
    _padding_0x018: [u32; 24],
    /// DDR port security identifiers.
    pub ddr_port_user_control: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, dma_secure_control), 0x0);
        assert_eq!(offset_of!(RegisterBlock, master_secure_control), 0x8);
        assert_eq!(offset_of!(RegisterBlock, nsaid_control0), 0x10);
        assert_eq!(offset_of!(RegisterBlock, nsaid_control1), 0x14);
        assert_eq!(offset_of!(RegisterBlock, ddr_port_user_control), 0x78);
        assert_eq!(size_of::<RegisterBlock>(), 0x7c);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
