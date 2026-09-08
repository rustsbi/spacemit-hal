//! K3 supervisor APLIC registers.

use crate::register::RWNoModify;
use volatile_register::{RW, WO};

// https://github.com/riscv/riscv-aia/blob/main/src/AdvPLIC.adoc
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/include/linux/irqchip/riscv-aplic.h
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/arch/riscv/boot/dts/spacemit/k3.dtsi
// The supervisor MSI domain exposes 512 wired sources; root MSI address and IDC registers are omitted.

/// K3 supervisor APLIC registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Domain configuration.
    pub domain_configuration: RW<u32>,
    /// Configuration for source IDs 1 through 512.
    pub source_configuration: [RW<u32>; 512],
    _padding_0x804: [u32; 1279],
    /// Pending bits; writing one sets a bit.
    pub set_pending: [RWNoModify<u32>; 17],
    _padding_0x1c44: [u32; 38],
    /// Set pending by source ID.
    pub set_pending_number: WO<u32>,
    _padding_0x1ce0: [u32; 8],
    /// Rectified inputs; writing one clears pending.
    pub clear_pending: [RWNoModify<u32>; 17],
    _padding_0x1d44: [u32; 38],
    /// Clear pending by source ID.
    pub clear_pending_number: WO<u32>,
    _padding_0x1de0: [u32; 8],
    /// Enable bits; writing one sets a bit.
    pub set_enable: [RWNoModify<u32>; 17],
    _padding_0x1e44: [u32; 38],
    /// Enable by source ID.
    pub set_enable_number: WO<u32>,
    _padding_0x1ee0: [u32; 8],
    /// Enable bits; writing one clears a bit.
    pub clear_enable: [RWNoModify<u32>; 17],
    _padding_0x1f44: [u32; 38],
    /// Disable by source ID.
    pub clear_enable_number: WO<u32>,
    _padding_0x1fe0: [u32; 8],
    /// Little-endian pending command.
    pub set_pending_number_le: WO<u32>,
    /// Big-endian pending command.
    pub set_pending_number_be: WO<u32>,
    _padding_0x2008: [u32; 1022],
    /// MSI command and busy status.
    pub generate_msi: RWNoModify<u32>,
    /// Targets for source IDs 1 through 512.
    pub target: [RW<u32>; 512],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, domain_configuration), 0x000);
        assert_eq!(offset_of!(RegisterBlock, source_configuration), 0x004);
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
        assert_eq!(size_of::<RegisterBlock>(), 0x3804);
    }
}
