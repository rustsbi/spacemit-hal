//! K1/M1 peripheral DMA registers.

use crate::register::RWNoModify;
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/dma/mmp_pdma.c
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/arch/riscv/boot/dts/spacemit/k1.dtsi
// Sixteen channels; request mappings are separately addressed at DRCMR(request) in the driver.
// Request routing tables are not copied from K3 without a K1 request-number map.

/// K1/M1 peripheral DMA registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Channel control and interrupt flags.
    pub channel_control: [RWNoModify<u32>; 16],
    _padding_0x040: [u32; 24],
    /// Channel alignment enables.
    pub alignment: RW<u32>,
    _padding_0x0a4: [u32; 19],
    /// Channel interrupt summary.
    pub interrupt_status: RO<u32>,
    _padding_0x0f4: [u32; 67],
    /// Channel descriptor, addresses and command.
    pub channel: [Channel; 16],
    /// Upper descriptor and address words.
    pub channel_high: [ChannelHigh; 16],
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
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0xf0);
        assert_eq!(offset_of!(RegisterBlock, channel), 0x200);
        assert_eq!(offset_of!(RegisterBlock, channel_high), 0x300);
        assert_eq!(size_of::<RegisterBlock>(), 0x400);
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
