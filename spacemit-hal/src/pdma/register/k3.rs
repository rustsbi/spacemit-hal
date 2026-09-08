//! K3 peripheral dma registers.

use crate::register::RWNoModify;
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/16_peripherals/dma.md
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/dma/mmp_pdma.c
// Only explicitly documented request mappings are exposed.

/// K3 peripheral dma registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Channel control, commands and W1C status (DCSR).
    pub channel_control: [RWNoModify<u32>; 16],
    _padding_0x040: [u32; 24],
    /// Per-channel byte alignment (DALGN).
    pub alignment: RW<u32>,
    /// Programmed-I/O bridge control and status (DPCSR).
    pub pio_control_status: RW<u32>,
    _padding_0x0a8: [u32; 14],
    /// Pending requests and clear command (DRQSR).
    pub request_status: RWNoModify<u32>,
    _padding_0x0e4: [u32; 3],
    /// Channel interrupt bitmap (DINT).
    pub interrupt_status: RO<u32>,
    _padding_0x0f4: [u32; 3],
    /// Documented request-to-channel mappings (DRCMR).
    pub request_map: [RW<u32>; 52],
    _padding_0x1d0: [u32; 12],
    /// Low address words and commands for each channel.
    pub channel: [Channel; 16],
    /// High address words for each channel.
    pub channel_high: [ChannelHigh; 16],
    _padding_0x400: [u32; 833],
    /// SSP0 transmit request mapping.
    pub ssp0_transmit_map: RW<u32>,
    /// SSP0 receive request mapping.
    pub ssp0_receive_map: RW<u32>,
    /// SSP1 transmit request mapping.
    pub ssp1_transmit_map: RW<u32>,
    /// SSP1 receive request mapping.
    pub ssp1_receive_map: RW<u32>,
    _padding_0x1114: [u32; 16],
    /// QSPI receive request mapping.
    pub qspi_receive_map: RW<u32>,
    /// QSPI transmit request mapping.
    pub qspi_transmit_map: RW<u32>,
}

/// One PDMA channel's transfer registers.
#[repr(C)]
pub struct Channel {
    /// Descriptor low address and stop flag.
    pub descriptor: RW<u32>,
    /// Source low address.
    pub source: RW<u32>,
    /// Destination low address.
    pub destination: RW<u32>,
    /// Transfer command.
    pub command: RW<u32>,
}

/// One PDMA channel's high address registers.
#[repr(C)]
pub struct ChannelHigh {
    /// Descriptor high address.
    pub descriptor: RW<u32>,
    /// Source high address.
    pub source: RW<u32>,
    /// Destination high address.
    pub destination: RW<u32>,
    _padding_0x00c: [u32; 1],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, channel_control), 0x0);
        assert_eq!(offset_of!(RegisterBlock, alignment), 0xa0);
        assert_eq!(offset_of!(RegisterBlock, pio_control_status), 0xa4);
        assert_eq!(offset_of!(RegisterBlock, request_status), 0xe0);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0xf0);
        assert_eq!(offset_of!(RegisterBlock, request_map), 0x100);
        assert_eq!(offset_of!(RegisterBlock, channel), 0x200);
        assert_eq!(offset_of!(RegisterBlock, channel_high), 0x300);
        assert_eq!(offset_of!(RegisterBlock, ssp0_transmit_map), 0x1104);
        assert_eq!(offset_of!(RegisterBlock, ssp0_receive_map), 0x1108);
        assert_eq!(offset_of!(RegisterBlock, ssp1_transmit_map), 0x110c);
        assert_eq!(offset_of!(RegisterBlock, ssp1_receive_map), 0x1110);
        assert_eq!(offset_of!(RegisterBlock, qspi_receive_map), 0x1154);
        assert_eq!(offset_of!(RegisterBlock, qspi_transmit_map), 0x1158);
        assert_eq!(size_of::<RegisterBlock>(), 0x115c);
        assert_eq!(align_of::<RegisterBlock>(), 4);
        assert_eq!(offset_of!(Channel, descriptor), 0x0);
        assert_eq!(offset_of!(Channel, source), 0x4);
        assert_eq!(offset_of!(Channel, destination), 0x8);
        assert_eq!(offset_of!(Channel, command), 0xc);
        assert_eq!(size_of::<Channel>(), 0x10);
        assert_eq!(align_of::<Channel>(), 4);
        assert_eq!(offset_of!(ChannelHigh, descriptor), 0x0);
        assert_eq!(offset_of!(ChannelHigh, source), 0x4);
        assert_eq!(offset_of!(ChannelHigh, destination), 0x8);
        assert_eq!(size_of::<ChannelHigh>(), 0x10);
        assert_eq!(align_of::<ChannelHigh>(), 4);
    }
}
