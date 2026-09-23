//! RISC-V IOMMU architectural register prefix, used by V100 (`RSCV0004`).
//!
//! Queue memory and device-directory management remain the caller's responsibility.
//! Only the architectural prefix is modeled; optional performance, debug and MSI
//! tables are not exposed. Perform 64-bit register accesses on RV64 only.
// https://github.com/riscv-non-isa/riscv-iommu/blob/main/src/iommu_registers.adoc

use crate::register::{RW1C, RWNoModify};
use volatile_register::{RO, RW};

/// Raw register layout; reserved and unsupported registers are not exposed.
#[repr(C)]
pub struct RegisterBlock {
    /// IOMMU capabilities; check before accessing optional features.
    pub capabilities: RO<u64>,
    /// Endian, interrupt mode and guest address width controls.
    pub feature_control: RW<u32>,
    _padding_0xc: [u8; 4],
    /// Device-directory-table pointer, translation mode and busy status.
    pub device_directory: RWNoModify<u64>,
    /// Command queue base and size.
    pub command_queue_base: RW<u64>,
    /// Hardware command consumer index.
    pub command_queue_head: RO<u32>,
    /// Software command producer index.
    pub command_queue_tail: RW<u32>,
    /// Fault queue base and size.
    pub fault_queue_base: RW<u64>,
    /// Software fault consumer index.
    pub fault_queue_head: RW<u32>,
    /// Hardware fault producer index.
    pub fault_queue_tail: RO<u32>,
    /// Optional ATS page-request queue base and size.
    pub page_request_queue_base: RW<u64>,
    /// Software page-request consumer index.
    pub page_request_queue_head: RW<u32>,
    /// Hardware page-request producer index.
    pub page_request_queue_tail: RO<u32>,
    /// Command queue control and W1C error status; write exact values.
    pub command_queue_control: RWNoModify<u32>,
    /// Fault queue control and W1C error status; write exact values.
    pub fault_queue_control: RWNoModify<u32>,
    /// Page-request queue control and W1C error status.
    pub page_request_queue_control: RWNoModify<u32>,
    /// Interrupt pending bits, acknowledged by writing one.
    pub interrupt_pending: RW1C<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, capabilities), 0x0);
        assert_eq!(offset_of!(RegisterBlock, feature_control), 0x8);
        assert_eq!(offset_of!(RegisterBlock, device_directory), 0x10);
        assert_eq!(offset_of!(RegisterBlock, command_queue_base), 0x18);
        assert_eq!(offset_of!(RegisterBlock, command_queue_head), 0x20);
        assert_eq!(offset_of!(RegisterBlock, command_queue_tail), 0x24);
        assert_eq!(offset_of!(RegisterBlock, fault_queue_base), 0x28);
        assert_eq!(offset_of!(RegisterBlock, fault_queue_head), 0x30);
        assert_eq!(offset_of!(RegisterBlock, fault_queue_tail), 0x34);
        assert_eq!(offset_of!(RegisterBlock, page_request_queue_base), 0x38);
        assert_eq!(offset_of!(RegisterBlock, page_request_queue_head), 0x40);
        assert_eq!(offset_of!(RegisterBlock, page_request_queue_tail), 0x44);
        assert_eq!(offset_of!(RegisterBlock, command_queue_control), 0x48);
        assert_eq!(offset_of!(RegisterBlock, fault_queue_control), 0x4c);
        assert_eq!(offset_of!(RegisterBlock, page_request_queue_control), 0x50);
        assert_eq!(offset_of!(RegisterBlock, interrupt_pending), 0x54);
        assert_eq!(size_of::<RegisterBlock>(), 0x58);
        assert_eq!(align_of::<RegisterBlock>(), 8);
    }
}
