//! K3 I/O physical-memory protection registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/15_security.md
// Section 15.4.4: register prefix common to all nine instances; table lengths depend on HWCFG.
// Source tables: base + 0x1000, stride 0x20; device-ID words: base + 0x9000, stride 4.
// Entry tables: base + ENTRYOFFSET (0xa000), stride 0x10; do not assume one instance's counts for another.

/// K3 I/O physical-memory protection registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Specification and vendor identification.
    pub version: RO<u32>,
    /// Implementation identification.
    pub implementation: RO<u32>,
    /// Capabilities and irreversible enable.
    pub hardware_config0: RWNoModify<u32>,
    /// Entry and requestor counts.
    pub hardware_config1: RO<u32>,
    /// Priority and translation capabilities.
    pub hardware_config2: RO<u32>,
    /// Entry-table offset.
    pub entry_offset: RO<u32>,
    _padding_0x018: [u32; 10],
    /// Write-one-set domain locks.
    pub domain_lock: RWNoModify<u32>,
    /// Upper write-one-set domain locks.
    pub domain_lock_high: RWNoModify<u32>,
    _padding_0x048: [u32; 1],
    /// Monotonic entry locks.
    pub entry_lock: RWNoModify<u32>,
    _padding_0x050: [u32; 4],
    /// Error reporting and write-one-set lock.
    pub error_config: RWNoModify<u32>,
    /// Fault information; bit zero acknowledges the fault.
    pub error_request_info: RW1C<u32>,
    /// Fault address bits 33:2.
    pub error_request_address: RO<u32>,
    /// Upper fault address.
    pub error_request_address_high: RO<u32>,
    /// Fault entry and requestor identifiers.
    pub error_request_id: RO<u32>,
    _padding_0x074: [u32; 35],
    /// Accumulated write faults.
    pub write_error_count: RO<u32>,
    /// Accumulated read faults.
    pub read_error_count: RO<u32>,
    _padding_0x108: [u32; 62],
    /// Device-ID selection and write-one-set lock.
    pub device_id_config: RWNoModify<u32>,
    _padding_0x204: [u32; 3],
    /// Default write address bits 33:12.
    pub default_write_address: RW<u32>,
    /// Upper default write address.
    pub default_write_address_high: RW<u32>,
    /// Default read address bits 33:12.
    pub default_read_address: RW<u32>,
    /// Upper default read address.
    pub default_read_address_high: RW<u32>,
}

/// One SID-to-domain mapping.
#[repr(C)]
pub struct SourceRegisters {
    /// Domain enables and write-one-set lock.
    pub enable: RWNoModify<u32>,
    /// Upper domain enables.
    pub enable_high: RW<u32>,
    _padding_0x008: [u32; 6],
}

/// One device-ID mapping.
#[repr(C)]
pub struct DeviceIdRegisters {
    /// Device identifier mapped to this SID.
    pub device_id: RW<u32>,
}

/// One physical-memory protection entry.
#[repr(C)]
pub struct EntryRegisters {
    /// Physical address and NAPOT size encoding.
    pub address: RW<u32>,
    /// Upper physical address.
    pub address_high: RW<u32>,
    /// Address matching and access permissions.
    pub config: RW<u32>,
    _padding_0x00c: [u32; 1],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, version), 0x0);
        assert_eq!(offset_of!(RegisterBlock, implementation), 0x4);
        assert_eq!(offset_of!(RegisterBlock, hardware_config0), 0x8);
        assert_eq!(offset_of!(RegisterBlock, hardware_config1), 0xc);
        assert_eq!(offset_of!(RegisterBlock, hardware_config2), 0x10);
        assert_eq!(offset_of!(RegisterBlock, entry_offset), 0x14);
        assert_eq!(offset_of!(RegisterBlock, domain_lock), 0x40);
        assert_eq!(offset_of!(RegisterBlock, domain_lock_high), 0x44);
        assert_eq!(offset_of!(RegisterBlock, entry_lock), 0x4c);
        assert_eq!(offset_of!(RegisterBlock, error_config), 0x60);
        assert_eq!(offset_of!(RegisterBlock, error_request_info), 0x64);
        assert_eq!(offset_of!(RegisterBlock, error_request_address), 0x68);
        assert_eq!(offset_of!(RegisterBlock, error_request_address_high), 0x6c);
        assert_eq!(offset_of!(RegisterBlock, error_request_id), 0x70);
        assert_eq!(offset_of!(RegisterBlock, write_error_count), 0x100);
        assert_eq!(offset_of!(RegisterBlock, read_error_count), 0x104);
        assert_eq!(offset_of!(RegisterBlock, device_id_config), 0x200);
        assert_eq!(offset_of!(RegisterBlock, default_write_address), 0x210);
        assert_eq!(offset_of!(RegisterBlock, default_write_address_high), 0x214);
        assert_eq!(offset_of!(RegisterBlock, default_read_address), 0x218);
        assert_eq!(offset_of!(RegisterBlock, default_read_address_high), 0x21c);
        assert_eq!(size_of::<RegisterBlock>(), 0x220);
        assert_eq!(align_of::<RegisterBlock>(), 4);
        assert_eq!(offset_of!(SourceRegisters, enable), 0x0);
        assert_eq!(offset_of!(SourceRegisters, enable_high), 0x4);
        assert_eq!(size_of::<SourceRegisters>(), 0x20);
        assert_eq!(align_of::<SourceRegisters>(), 4);
        assert_eq!(offset_of!(DeviceIdRegisters, device_id), 0x0);
        assert_eq!(size_of::<DeviceIdRegisters>(), 0x4);
        assert_eq!(align_of::<DeviceIdRegisters>(), 4);
        assert_eq!(offset_of!(EntryRegisters, address), 0x0);
        assert_eq!(offset_of!(EntryRegisters, address_high), 0x4);
        assert_eq!(offset_of!(EntryRegisters, config), 0x8);
        assert_eq!(size_of::<EntryRegisters>(), 0x10);
        assert_eq!(align_of::<EntryRegisters>(), 4);
    }
}
