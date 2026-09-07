//! K1/M1 CIU register layout.

use super::ResetVector;

// CPU Config Unit: base 0xd428_2c00, size 0x400; C0/C1_RVBADDR_LO/HI_ADDR.
// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k1/k1_docs/k1_usermanual/6.Address_Mapping.md
// https://github.com/spacemit-com/opensbi/blob/fc02b891b17b8bdc1273a39f80aa374cd99ba9a2/platform/generic/include/spacemit/k1x/k1x_evb.h

/// K1/M1 CIU registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0x000: [u32; 108],
    /// Cluster 0 reset-vector address.
    pub cluster0_reset_vector: ResetVector,
    _reserved_0x1b8: [u32; 62],
    /// Cluster 1 reset-vector address.
    pub cluster1_reset_vector: ResetVector,
    _reserved_0x2b8: [u32; 82],
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, cluster0_reset_vector), 0x1b0);
        assert_eq!(offset_of!(RegisterBlock, cluster1_reset_vector), 0x2b0);
        assert_eq!(size_of::<RegisterBlock>(), 0x400);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
