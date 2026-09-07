use volatile_register::RW;

// Vendor firmware accesses RVBADDR as two 32-bit read/write registers.
// https://github.com/spacemit-com/esos/blob/eaf9afd83b27583b9bcafe0153028b32219518f5/bsp/spacemit/drivers/rpmi/k3/k3-os0_hsm_interface.c

/// Cluster reset-vector address registers.
#[repr(C)]
pub struct ResetVector {
    /// Reset-vector address bits 31:0.
    pub low: RW<u32>,
    /// Reset-vector address upper word.
    pub high: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::ResetVector;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(ResetVector, low), 0);
        assert_eq!(offset_of!(ResetVector, high), 4);
        assert_eq!(size_of::<ResetVector>(), 8);
        assert_eq!(align_of::<ResetVector>(), 4);
    }
}
