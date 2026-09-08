//! K1/M1 and K3 DWC3 global and device registers.

use crate::register::RWNoModify;
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/usb/dwc3/core.h
// Offsets are relative to the controller resource, not GLOBALS_REGS_START.
// Only PHY0, event buffer 0 and control endpoint commands are exposed.

/// K1/M1 and K3 DWC3 global and device registers.
#[repr(C)]
pub struct RegisterBlock {
    _padding_0x000: [u32; 12352],
    /// bus configuration0.
    pub bus_configuration0: RW<u32>,
    /// bus configuration1.
    pub bus_configuration1: RW<u32>,
    /// transmit threshold.
    pub transmit_threshold: RW<u32>,
    /// receive threshold.
    pub receive_threshold: RW<u32>,
    /// global control.
    pub global_control: RWNoModify<u32>,
    /// global event enable.
    pub global_event_enable: RW<u32>,
    /// global status.
    pub global_status: RWNoModify<u32>,
    /// user control1.
    pub user_control1: RW<u32>,
    /// core id.
    pub core_id: RO<u32>,
    /// gpio.
    pub gpio: RW<u32>,
    /// user id.
    pub user_id: RW<u32>,
    /// user control.
    pub user_control: RW<u32>,
    /// bus error address low.
    pub bus_error_address_low: RO<u32>,
    /// bus error address high.
    pub bus_error_address_high: RO<u32>,
    /// port bitmap low.
    pub port_bitmap_low: RW<u32>,
    /// port bitmap high.
    pub port_bitmap_high: RW<u32>,
    /// Hardware capability words.
    pub hardware_parameters: [RO<u32>; 8],
    _padding_0xc160: [u32; 40],
    /// usb2 phy configuration.
    pub usb2_phy_configuration: RWNoModify<u32>,
    _padding_0xc204: [u32; 47],
    /// usb3 pipe control.
    pub usb3_pipe_control: RWNoModify<u32>,
    _padding_0xc2c4: [u32; 15],
    /// transmit fifo0 size.
    pub transmit_fifo0_size: RW<u32>,
    _padding_0xc304: [u32; 31],
    /// receive fifo0 size.
    pub receive_fifo0_size: RW<u32>,
    _padding_0xc384: [u32; 31],
    /// event0 address low.
    pub event0_address_low: RW<u32>,
    /// event0 address high.
    pub event0_address_high: RW<u32>,
    /// event0 size.
    pub event0_size: RW<u32>,
    /// Reading returns pending bytes; writing consumes that many bytes.
    pub event0_count: RWNoModify<u32>,
    _padding_0xc410: [u32; 124],
    /// hardware parameters8.
    pub hardware_parameters8: RO<u32>,
    _padding_0xc604: [u32; 11],
    /// frame length adjustment.
    pub frame_length_adjustment: RW<u32>,
    _padding_0xc634: [u32; 51],
    /// device configuration.
    pub device_configuration: RW<u32>,
    /// device control.
    pub device_control: RWNoModify<u32>,
    /// device event enable.
    pub device_event_enable: RW<u32>,
    /// device status.
    pub device_status: RO<u32>,
    /// device command parameter.
    pub device_command_parameter: RW<u32>,
    /// device command.
    pub device_command: RWNoModify<u32>,
    _padding_0xc718: [u32; 2],
    /// active endpoint enable.
    pub active_endpoint_enable: RW<u32>,
    _padding_0xc724: [u32; 55],
    /// EP0 OUT and IN command registers.
    pub control_endpoint: [Endpoint; 2],
}

/// DWC3 endpoint command registers.
#[repr(C)]
pub struct Endpoint {
    /// parameter2.
    pub parameter2: RW<u32>,
    /// parameter1.
    pub parameter1: RW<u32>,
    /// parameter0.
    pub parameter0: RW<u32>,
    /// command.
    pub command: RWNoModify<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, bus_configuration0), 0xc100);
        assert_eq!(offset_of!(RegisterBlock, bus_configuration1), 0xc104);
        assert_eq!(offset_of!(RegisterBlock, transmit_threshold), 0xc108);
        assert_eq!(offset_of!(RegisterBlock, receive_threshold), 0xc10c);
        assert_eq!(offset_of!(RegisterBlock, global_control), 0xc110);
        assert_eq!(offset_of!(RegisterBlock, global_event_enable), 0xc114);
        assert_eq!(offset_of!(RegisterBlock, global_status), 0xc118);
        assert_eq!(offset_of!(RegisterBlock, user_control1), 0xc11c);
        assert_eq!(offset_of!(RegisterBlock, core_id), 0xc120);
        assert_eq!(offset_of!(RegisterBlock, gpio), 0xc124);
        assert_eq!(offset_of!(RegisterBlock, user_id), 0xc128);
        assert_eq!(offset_of!(RegisterBlock, user_control), 0xc12c);
        assert_eq!(offset_of!(RegisterBlock, bus_error_address_low), 0xc130);
        assert_eq!(offset_of!(RegisterBlock, bus_error_address_high), 0xc134);
        assert_eq!(offset_of!(RegisterBlock, port_bitmap_low), 0xc138);
        assert_eq!(offset_of!(RegisterBlock, port_bitmap_high), 0xc13c);
        assert_eq!(offset_of!(RegisterBlock, hardware_parameters), 0xc140);
        assert_eq!(offset_of!(RegisterBlock, usb2_phy_configuration), 0xc200);
        assert_eq!(offset_of!(RegisterBlock, usb3_pipe_control), 0xc2c0);
        assert_eq!(offset_of!(RegisterBlock, transmit_fifo0_size), 0xc300);
        assert_eq!(offset_of!(RegisterBlock, receive_fifo0_size), 0xc380);
        assert_eq!(offset_of!(RegisterBlock, event0_address_low), 0xc400);
        assert_eq!(offset_of!(RegisterBlock, event0_address_high), 0xc404);
        assert_eq!(offset_of!(RegisterBlock, event0_size), 0xc408);
        assert_eq!(offset_of!(RegisterBlock, event0_count), 0xc40c);
        assert_eq!(offset_of!(RegisterBlock, hardware_parameters8), 0xc600);
        assert_eq!(offset_of!(RegisterBlock, frame_length_adjustment), 0xc630);
        assert_eq!(offset_of!(RegisterBlock, device_configuration), 0xc700);
        assert_eq!(offset_of!(RegisterBlock, device_control), 0xc704);
        assert_eq!(offset_of!(RegisterBlock, device_event_enable), 0xc708);
        assert_eq!(offset_of!(RegisterBlock, device_status), 0xc70c);
        assert_eq!(offset_of!(RegisterBlock, device_command_parameter), 0xc710);
        assert_eq!(offset_of!(RegisterBlock, device_command), 0xc714);
        assert_eq!(offset_of!(RegisterBlock, active_endpoint_enable), 0xc720);
        assert_eq!(offset_of!(RegisterBlock, control_endpoint), 0xc800);
        assert_eq!(size_of::<Endpoint>(), 0x10);
        assert_eq!(size_of::<RegisterBlock>(), 0xc820);
    }
}
