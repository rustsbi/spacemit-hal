//! K3 DesignWare Ethernet QoS registers.

use crate::register::RW1C;
use crate::register::{RC, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/net/ethernet/stmicro/stmmac/dwmac4.h
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/net/ethernet/stmicro/stmmac/dwmac4_dma.h
// Only the four documented queues/channels and primary MAC address are exposed.

/// K3 DesignWare Ethernet QoS registers.
#[repr(C)]
pub struct RegisterBlock {
    /// configuration.
    pub configuration: RW<u32>,
    /// extended configuration.
    pub extended_configuration: RW<u32>,
    /// packet filter.
    pub packet_filter: RW<u32>,
    _padding_0x00c: [u32; 25],
    /// Transmit queue flow controls.
    pub transmit_flow_control: [RW<u32>; 4],
    _padding_0x080: [u32; 4],
    /// receive flow control.
    pub receive_flow_control: RW<u32>,
    _padding_0x094: [u32; 1],
    /// transmit priority map0.
    pub transmit_priority_map0: RW<u32>,
    /// transmit priority map1.
    pub transmit_priority_map1: RW<u32>,
    /// receive queue control0.
    pub receive_queue_control0: RW<u32>,
    /// receive queue control1.
    pub receive_queue_control1: RW<u32>,
    /// receive queue control2.
    pub receive_queue_control2: RW<u32>,
    /// receive queue control3.
    pub receive_queue_control3: RW<u32>,
    /// interrupt status.
    pub interrupt_status: RO<u32>,
    /// interrupt enable.
    pub interrupt_enable: RW<u32>,
    _padding_0x0b8: [u32; 2],
    /// power management.
    pub power_management: RWNoModify<u32>,
    _padding_0x0c4: [u32; 13],
    /// phy interface status.
    pub phy_interface_status: RC<u32>,
    _padding_0x0fc: [u32; 6],
    /// debug.
    pub debug: RO<u32>,
    _padding_0x118: [u32; 1],
    /// Hardware feature words.
    pub hardware_features: [RO<u32>; 4],
    _padding_0x12c: [u32; 53],
    /// mdio address.
    pub mdio_address: RWNoModify<u32>,
    /// mdio data.
    pub mdio_data: RW<u32>,
    _padding_0x208: [u32; 2],
    /// arp address.
    pub arp_address: RW<u32>,
    _padding_0x214: [u32; 59],
    /// mac address high.
    pub mac_address_high: RW<u32>,
    /// mac address low.
    pub mac_address_low: RW<u32>,
    _padding_0x308: [u32; 574],
    /// mtl operation mode.
    pub mtl_operation_mode: RW<u32>,
    _padding_0xc04: [u32; 7],
    /// mtl interrupt status.
    pub mtl_interrupt_status: RO<u32>,
    _padding_0xc24: [u32; 3],
    /// receive dma map0.
    pub receive_dma_map0: RW<u32>,
    _padding_0xc34: [u32; 243],
    /// dma bus mode.
    pub dma_bus_mode: RWNoModify<u32>,
    /// dma system bus mode.
    pub dma_system_bus_mode: RW<u32>,
    /// dma status.
    pub dma_status: RO<u32>,
    _padding_0x100c: [u32; 7],
    /// dma axi bus mode.
    pub dma_axi_bus_mode: RW<u32>,
    _padding_0x102c: [u32; 53],
    /// DMA channels.
    pub dma_channel: [DmaChannel; 4],
}

/// Ethernet DMA channel registers.
#[repr(C)]
pub struct DmaChannel {
    /// control.
    pub control: RW<u32>,
    /// transmit control.
    pub transmit_control: RW<u32>,
    /// receive control.
    pub receive_control: RW<u32>,
    _padding_0x00c: [u32; 1],
    /// transmit descriptor high.
    pub transmit_descriptor_high: RW<u32>,
    /// transmit descriptor low.
    pub transmit_descriptor_low: RW<u32>,
    /// receive descriptor high.
    pub receive_descriptor_high: RW<u32>,
    /// receive descriptor low.
    pub receive_descriptor_low: RW<u32>,
    /// transmit tail.
    pub transmit_tail: RWNoModify<u32>,
    _padding_0x024: [u32; 1],
    /// receive tail.
    pub receive_tail: RWNoModify<u32>,
    /// transmit ring length.
    pub transmit_ring_length: RW<u32>,
    /// receive ring length.
    pub receive_ring_length: RW<u32>,
    /// interrupt enable.
    pub interrupt_enable: RW<u32>,
    /// receive watchdog.
    pub receive_watchdog: RW<u32>,
    /// slot control status.
    pub slot_control_status: RWNoModify<u32>,
    _padding_0x040: [u32; 1],
    /// current transmit descriptor.
    pub current_transmit_descriptor: RO<u32>,
    _padding_0x048: [u32; 1],
    /// current receive descriptor.
    pub current_receive_descriptor: RO<u32>,
    /// current transmit buffer high.
    pub current_transmit_buffer_high: RO<u32>,
    /// current transmit buffer low.
    pub current_transmit_buffer_low: RO<u32>,
    /// current receive buffer high.
    pub current_receive_buffer_high: RO<u32>,
    /// current receive buffer low.
    pub current_receive_buffer_low: RO<u32>,
    /// status.
    pub status: RW1C<u32>,
    _padding_0x064: [u32; 7],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, configuration), 0x000);
        assert_eq!(offset_of!(RegisterBlock, extended_configuration), 0x004);
        assert_eq!(offset_of!(RegisterBlock, packet_filter), 0x008);
        assert_eq!(offset_of!(RegisterBlock, transmit_flow_control), 0x070);
        assert_eq!(offset_of!(RegisterBlock, receive_flow_control), 0x090);
        assert_eq!(offset_of!(RegisterBlock, transmit_priority_map0), 0x098);
        assert_eq!(offset_of!(RegisterBlock, transmit_priority_map1), 0x09c);
        assert_eq!(offset_of!(RegisterBlock, receive_queue_control0), 0x0a0);
        assert_eq!(offset_of!(RegisterBlock, receive_queue_control1), 0x0a4);
        assert_eq!(offset_of!(RegisterBlock, receive_queue_control2), 0x0a8);
        assert_eq!(offset_of!(RegisterBlock, receive_queue_control3), 0x0ac);
        assert_eq!(offset_of!(RegisterBlock, interrupt_status), 0x0b0);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x0b4);
        assert_eq!(offset_of!(RegisterBlock, power_management), 0x0c0);
        assert_eq!(offset_of!(RegisterBlock, phy_interface_status), 0x0f8);
        assert_eq!(offset_of!(RegisterBlock, debug), 0x114);
        assert_eq!(offset_of!(RegisterBlock, hardware_features), 0x11c);
        assert_eq!(offset_of!(RegisterBlock, mdio_address), 0x200);
        assert_eq!(offset_of!(RegisterBlock, mdio_data), 0x204);
        assert_eq!(offset_of!(RegisterBlock, arp_address), 0x210);
        assert_eq!(offset_of!(RegisterBlock, mac_address_high), 0x300);
        assert_eq!(offset_of!(RegisterBlock, mac_address_low), 0x304);
        assert_eq!(offset_of!(RegisterBlock, mtl_operation_mode), 0xc00);
        assert_eq!(offset_of!(RegisterBlock, mtl_interrupt_status), 0xc20);
        assert_eq!(offset_of!(RegisterBlock, receive_dma_map0), 0xc30);
        assert_eq!(offset_of!(RegisterBlock, dma_bus_mode), 0x1000);
        assert_eq!(offset_of!(RegisterBlock, dma_system_bus_mode), 0x1004);
        assert_eq!(offset_of!(RegisterBlock, dma_status), 0x1008);
        assert_eq!(offset_of!(RegisterBlock, dma_axi_bus_mode), 0x1028);
        assert_eq!(offset_of!(RegisterBlock, dma_channel), 0x1100);
        assert_eq!(size_of::<DmaChannel>(), 0x80);
        assert_eq!(offset_of!(DmaChannel, status), 0x60);
        assert_eq!(size_of::<RegisterBlock>(), 0x1300);
    }
}
