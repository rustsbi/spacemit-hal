//! K3 UFS host and PHY-management registers.

use crate::register::{RC, RWNoModify};
use volatile_register::{RO, RW, WO};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/09_memory_storage.md
// Section 9.7.5; host and PHY management have separate MMIO bases.

/// K3 UFS host registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Controller capabilities.
    pub capabilities: RO<u32>,
    _padding_0x004: [u32; 1],
    /// UFSHCI version.
    pub version: RO<u32>,
    _padding_0x00c: [u32; 1],
    /// Controller product ID.
    pub product_id: RO<u32>,
    /// Controller manufacturer ID.
    pub manufacturer_id: RO<u32>,
    /// Auto-hibernate idle timer.
    pub auto_hibernate_timer: RW<u32>,
    _padding_0x01c: [u32; 1],
    /// Interrupt status; write an acknowledgement mask.
    pub interrupt_status: RWNoModify<u32>,
    /// Interrupt enables; the manual's RWC label is ambiguous.
    pub interrupt_enable: RWNoModify<u32>,
    _padding_0x028: [u32; 2],
    /// Controller status.
    pub status: RO<u32>,
    /// Controller enable.
    pub enable: RW<u32>,
    /// PHY error; reading clears the valid flag.
    pub phy_error: RWNoModify<u32>,
    /// Read-clear data-link error.
    pub data_link_error: RC<u32>,
    /// Read-clear network error.
    pub network_error: RC<u32>,
    /// Read-clear transport error.
    pub transport_error: RC<u32>,
    /// Read-clear DME error.
    pub dme_error: RC<u32>,
    /// Interrupt aggregation configuration and counter-reset command.
    pub interrupt_aggregation: RWNoModify<u32>,
    /// Transfer descriptor base address.
    pub transfer_list_base: RW<u32>,
    /// Transfer descriptor base high address.
    pub transfer_list_base_high: RW<u32>,
    /// Write-one-to-set transfer doorbell.
    pub transfer_doorbell: RWNoModify<u32>,
    /// Transfer-list clear command.
    pub transfer_clear: WO<u32>,
    /// Transfer-list run/stop control.
    pub transfer_run_stop: RW<u32>,
    /// Transfer completion acknowledgements.
    pub transfer_completion: RWNoModify<u32>,
    _padding_0x068: [u32; 2],
    /// Task descriptor base address.
    pub task_list_base: RW<u32>,
    /// Task descriptor base high address.
    pub task_list_base_high: RW<u32>,
    /// Write-one-to-set task doorbell.
    pub task_doorbell: RWNoModify<u32>,
    /// Task-list clear command.
    pub task_clear: RWNoModify<u32>,
    /// Task-list run/stop control.
    pub task_run_stop: RW<u32>,
    _padding_0x084: [u32; 3],
    /// UIC command launch.
    pub uic_command: RWNoModify<u32>,
    /// UIC argument 1.
    pub uic_argument: RW<u32>,
    /// UIC argument 2 and result.
    pub uic_argument2: RW<u32>,
    /// UIC argument 3 and result.
    pub uic_argument3: RW<u32>,
    _padding_0x0a0: [u32; 8],
    /// System-clock cycles per microsecond.
    pub system_clock_1us: RW<u32>,
    /// TX symbol clock period.
    pub tx_symbol_clock: RW<u32>,
    /// Local CPort ID.
    pub local_cport_id: RW<u32>,
    /// Read-clear PA error code.
    pub pa_error_code: RC<u32>,
    /// Retry timer.
    pub retry_timer: RW<u32>,
    _padding_0x0d4: [u32; 1],
    /// PA link-startup timer.
    pub link_startup_timer: RW<u32>,
}

/// UFS PHY-management registers.
#[repr(C)]
pub struct ManagementRegisters {
    /// PHY reset control.
    pub phy_reset: RW<u32>,
    /// PHY power and PLL-lock status.
    pub phy_power: RW<u32>,
    /// PHY backdoor access enable.
    pub phy_backdoor: RW<u32>,
    /// Device reset and reference-clock control.
    pub device_io: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, capabilities), 0x0);
        assert_eq!(offset_of!(RegisterBlock, version), 0x8);
        assert_eq!(offset_of!(RegisterBlock, product_id), 0x10);
        assert_eq!(offset_of!(RegisterBlock, manufacturer_id), 0x14);
        assert_eq!(offset_of!(RegisterBlock, auto_hibernate_timer), 0x18);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x20);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x24);
        assert_eq!(offset_of!(RegisterBlock, status), 0x30);
        assert_eq!(offset_of!(RegisterBlock, enable), 0x34);
        assert_eq!(offset_of!(RegisterBlock, phy_error), 0x38);
        assert_eq!(offset_of!(RegisterBlock, data_link_error), 0x3c);
        assert_eq!(offset_of!(RegisterBlock, network_error), 0x40);
        assert_eq!(offset_of!(RegisterBlock, transport_error), 0x44);
        assert_eq!(offset_of!(RegisterBlock, dme_error), 0x48);
        assert_eq!(offset_of!(RegisterBlock, interrupt_aggregation), 0x4c);
        assert_eq!(offset_of!(RegisterBlock, transfer_list_base), 0x50);
        assert_eq!(offset_of!(RegisterBlock, transfer_list_base_high), 0x54);
        assert_eq!(offset_of!(RegisterBlock, transfer_doorbell), 0x58);
        assert_eq!(offset_of!(RegisterBlock, transfer_clear), 0x5c);
        assert_eq!(offset_of!(RegisterBlock, transfer_run_stop), 0x60);
        assert_eq!(offset_of!(RegisterBlock, transfer_completion), 0x64);
        assert_eq!(offset_of!(RegisterBlock, task_list_base), 0x70);
        assert_eq!(offset_of!(RegisterBlock, task_list_base_high), 0x74);
        assert_eq!(offset_of!(RegisterBlock, task_doorbell), 0x78);
        assert_eq!(offset_of!(RegisterBlock, task_clear), 0x7c);
        assert_eq!(offset_of!(RegisterBlock, task_run_stop), 0x80);
        assert_eq!(offset_of!(RegisterBlock, uic_command), 0x90);
        assert_eq!(offset_of!(RegisterBlock, uic_argument), 0x94);
        assert_eq!(offset_of!(RegisterBlock, uic_argument2), 0x98);
        assert_eq!(offset_of!(RegisterBlock, uic_argument3), 0x9c);
        assert_eq!(offset_of!(RegisterBlock, system_clock_1us), 0xc0);
        assert_eq!(offset_of!(RegisterBlock, tx_symbol_clock), 0xc4);
        assert_eq!(offset_of!(RegisterBlock, local_cport_id), 0xc8);
        assert_eq!(offset_of!(RegisterBlock, pa_error_code), 0xcc);
        assert_eq!(offset_of!(RegisterBlock, retry_timer), 0xd0);
        assert_eq!(offset_of!(RegisterBlock, link_startup_timer), 0xd8);
        assert_eq!(size_of::<RegisterBlock>(), 0xdc);
        assert_eq!(align_of::<RegisterBlock>(), 4);
        assert_eq!(offset_of!(ManagementRegisters, phy_reset), 0x0);
        assert_eq!(offset_of!(ManagementRegisters, phy_power), 0x4);
        assert_eq!(offset_of!(ManagementRegisters, phy_backdoor), 0x8);
        assert_eq!(offset_of!(ManagementRegisters, device_io), 0xc);
        assert_eq!(size_of::<ManagementRegisters>(), 0x10);
        assert_eq!(align_of::<ManagementRegisters>(), 4);
    }
}
