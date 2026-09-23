//! V100 ACPI-described APLIC MSI domains (`RSCV0002`).
//!
//! Each domain has 511 sources, not the K3 layout's 512. Root MSI-address
//! registers and direct-delivery contexts are not exposed by this S-mode map.
// https://github.com/riscv/riscv-aia/blob/main/src/AdvPLIC.adoc

use crate::register::RWNoModify;
use volatile_register::{RW, WO};

/// Raw register layout; reserved and unsupported registers are not exposed.
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt domain configuration.
    pub domain_configuration: RW<u32>,
    /// Configuration for source IDs 1 through 511 (index = ID - 1).
    pub source_configuration: [RW<u32>; 511],
    _padding_0x800: [u8; 5120],
    /// Bitmap command/status registers; bit zero of word zero is reserved.
    pub set_pending: [RWNoModify<u32>; 16],
    _padding_0x1c40: [u8; 156],
    /// Apply the command to one source ID.
    pub set_pending_number: WO<u32>,
    _padding_0x1ce0: [u8; 32],
    /// Bitmap command/status registers; bit zero of word zero is reserved.
    pub clear_pending: [RWNoModify<u32>; 16],
    _padding_0x1d40: [u8; 156],
    /// Apply the command to one source ID.
    pub clear_pending_number: WO<u32>,
    _padding_0x1de0: [u8; 32],
    /// Bitmap command/status registers; bit zero of word zero is reserved.
    pub set_enable: [RWNoModify<u32>; 16],
    _padding_0x1e40: [u8; 156],
    /// Apply the command to one source ID.
    pub set_enable_number: WO<u32>,
    _padding_0x1ee0: [u8; 32],
    /// Bitmap command/status registers; bit zero of word zero is reserved.
    pub clear_enable: [RWNoModify<u32>; 16],
    _padding_0x1f40: [u8; 156],
    /// Apply the command to one source ID.
    pub clear_enable_number: WO<u32>,
    _padding_0x1fe0: [u8; 32],
    /// Little-endian pending command.
    pub set_pending_number_le: WO<u32>,
    /// Big-endian pending command.
    pub set_pending_number_be: WO<u32>,
    _padding_0x2008: [u8; 4088],
    /// MSI command and busy status.
    pub generate_msi: RWNoModify<u32>,
    /// MSI targets for source IDs 1 through 511.
    pub target: [RW<u32>; 511],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, domain_configuration), 0x0);
        assert_eq!(offset_of!(RegisterBlock, source_configuration), 0x4);
        assert_eq!(offset_of!(RegisterBlock, set_pending), 0x1c00);
        assert_eq!(offset_of!(RegisterBlock, set_pending_number), 0x1cdc);
        assert_eq!(offset_of!(RegisterBlock, clear_pending), 0x1d00);
        assert_eq!(offset_of!(RegisterBlock, clear_pending_number), 0x1ddc);
        assert_eq!(offset_of!(RegisterBlock, set_enable), 0x1e00);
        assert_eq!(offset_of!(RegisterBlock, set_enable_number), 0x1edc);
        assert_eq!(offset_of!(RegisterBlock, clear_enable), 0x1f00);
        assert_eq!(offset_of!(RegisterBlock, clear_enable_number), 0x1fdc);
        assert_eq!(offset_of!(RegisterBlock, set_pending_number_le), 0x2000);
        assert_eq!(offset_of!(RegisterBlock, set_pending_number_be), 0x2004);
        assert_eq!(offset_of!(RegisterBlock, generate_msi), 0x3000);
        assert_eq!(offset_of!(RegisterBlock, target), 0x3004);
        assert_eq!(size_of::<RegisterBlock>(), 0x3800);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
