//! K3 CIU register layout.

use super::ResetVector;

// CPU Config Unit: base 0xd428_2c00, size 0x400; C0-C3_RVBADDR_LO/HI_ADDR.
// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/06_address_map.md
// https://github.com/spacemit-com/opensbi/blob/8bd2cbdf9856dbc1a990d36e26bf47411f356c42/platform/generic/include/spacemit/k3/k3.h

/// K3 CIU registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0x000: [u32; 108],
    /// Cluster 0 reset-vector address.
    pub cluster0_reset_vector: ResetVector,
    _reserved_0x1b8: [u32; 42],
    /// Cluster 3 reset-vector address.
    pub cluster3_reset_vector: ResetVector,
    _reserved_0x268: [u32; 18],
    /// Cluster 1 reset-vector address.
    pub cluster1_reset_vector: ResetVector,
    _reserved_0x2b8: [u32; 76],
    /// Cluster 2 reset-vector address.
    pub cluster2_reset_vector: ResetVector,
    _reserved_0x3f0: [u32; 4],
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_block_layout() {
        assert_eq!(offset_of!(RegisterBlock, cluster0_reset_vector), 0x1b0);
        assert_eq!(offset_of!(RegisterBlock, cluster1_reset_vector), 0x2b0);
        assert_eq!(offset_of!(RegisterBlock, cluster2_reset_vector), 0x3e8);
        assert_eq!(offset_of!(RegisterBlock, cluster3_reset_vector), 0x260);
        assert_eq!(size_of::<RegisterBlock>(), 0x400);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
