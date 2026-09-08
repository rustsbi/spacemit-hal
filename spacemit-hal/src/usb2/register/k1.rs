//! K1/M1 USB2 capability registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::RO;
use volatile_register::RW;

// https://github.com/spacemit-com/linux-6.6/blob/k1-bl-v2.2.y/drivers/usb/gadget/udc/mv_udc.h
// The resource begins at 0xc0900100 (OTG) or 0xc0980100 (host).
// Operational registers begin at resource + CAPLENGTH[7:0], not a fixed offset.

/// K1/M1 USB2 capability registers.
#[repr(C)]
pub struct RegisterBlock {
    /// length version.
    pub length_version: RO<u32>,
    /// host structural parameters.
    pub host_structural_parameters: RO<u32>,
    /// host capability parameters.
    pub host_capability_parameters: RO<u32>,
    _padding_0x00c: [u32; 5],
    /// device version.
    pub device_version: RO<u32>,
    /// device capability parameters.
    pub device_capability_parameters: RO<u32>,
}

/// USB2 operational registers at the CAPLENGTH-selected offset.
#[repr(C)]
pub struct OperationalRegisters {
    /// command.
    pub command: RWNoModify<u32>,
    /// status.
    pub status: RW1C<u32>,
    /// interrupt enable.
    pub interrupt_enable: RW<u32>,
    /// frame index.
    pub frame_index: RW<u32>,
    _padding_0x010: [u32; 1],
    /// device address.
    pub device_address: RW<u32>,
    /// endpoint list address.
    pub endpoint_list_address: RW<u32>,
    /// tt control.
    pub tt_control: RW<u32>,
    /// burst size.
    pub burst_size: RW<u32>,
    /// transmit tuning.
    pub transmit_tuning: RW<u32>,
    _padding_0x028: [u32; 4],
    /// endpoint nak.
    pub endpoint_nak: RW1C<u32>,
    /// endpoint nak enable.
    pub endpoint_nak_enable: RW<u32>,
    /// configured flag.
    pub configured_flag: RW<u32>,
    /// Host port controls.
    pub port_control: [RWNoModify<u32>; 8],
    /// otg control.
    pub otg_control: RWNoModify<u32>,
    /// mode.
    pub mode: RW<u32>,
    /// setup status.
    pub setup_status: RW1C<u32>,
    /// endpoint prime.
    pub endpoint_prime: RWNoModify<u32>,
    /// endpoint flush.
    pub endpoint_flush: RWNoModify<u32>,
    /// endpoint status.
    pub endpoint_status: RO<u32>,
    /// endpoint complete.
    pub endpoint_complete: RW1C<u32>,
    /// Endpoint controls.
    pub endpoint_control: [RW<u32>; 16],
    /// mux control.
    pub mux_control: RW<u32>,
    /// interrupt status.
    pub interrupt_status: RWNoModify<u32>,
    /// extended interrupt enable.
    pub extended_interrupt_enable: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(OperationalRegisters, command), 0x000);
        assert_eq!(offset_of!(OperationalRegisters, status), 0x004);
        assert_eq!(offset_of!(OperationalRegisters, interrupt_enable), 0x008);
        assert_eq!(offset_of!(OperationalRegisters, frame_index), 0x00c);
        assert_eq!(offset_of!(OperationalRegisters, device_address), 0x014);
        assert_eq!(
            offset_of!(OperationalRegisters, endpoint_list_address),
            0x018
        );
        assert_eq!(offset_of!(OperationalRegisters, tt_control), 0x01c);
        assert_eq!(offset_of!(OperationalRegisters, burst_size), 0x020);
        assert_eq!(offset_of!(OperationalRegisters, transmit_tuning), 0x024);
        assert_eq!(offset_of!(OperationalRegisters, endpoint_nak), 0x038);
        assert_eq!(offset_of!(OperationalRegisters, endpoint_nak_enable), 0x03c);
        assert_eq!(offset_of!(OperationalRegisters, configured_flag), 0x040);
        assert_eq!(offset_of!(OperationalRegisters, port_control), 0x044);
        assert_eq!(offset_of!(OperationalRegisters, otg_control), 0x064);
        assert_eq!(offset_of!(OperationalRegisters, mode), 0x068);
        assert_eq!(offset_of!(OperationalRegisters, setup_status), 0x06c);
        assert_eq!(offset_of!(OperationalRegisters, endpoint_prime), 0x070);
        assert_eq!(offset_of!(OperationalRegisters, endpoint_flush), 0x074);
        assert_eq!(offset_of!(OperationalRegisters, endpoint_status), 0x078);
        assert_eq!(offset_of!(OperationalRegisters, endpoint_complete), 0x07c);
        assert_eq!(offset_of!(OperationalRegisters, endpoint_control), 0x080);
        assert_eq!(offset_of!(OperationalRegisters, mux_control), 0x0c0);
        assert_eq!(offset_of!(OperationalRegisters, interrupt_status), 0x0c4);
        assert_eq!(
            offset_of!(OperationalRegisters, extended_interrupt_enable),
            0x0c8
        );
        assert_eq!(size_of::<OperationalRegisters>(), 0x0cc);
        assert_eq!(offset_of!(RegisterBlock, length_version), 0x000);
        assert_eq!(offset_of!(RegisterBlock, host_structural_parameters), 0x004);
        assert_eq!(offset_of!(RegisterBlock, host_capability_parameters), 0x008);
        assert_eq!(offset_of!(RegisterBlock, device_version), 0x020);
        assert_eq!(
            offset_of!(RegisterBlock, device_capability_parameters),
            0x024
        );
        assert_eq!(size_of::<RegisterBlock>(), 0x028);
    }
}
