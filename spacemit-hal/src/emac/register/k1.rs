//! K1/M1 Ethernet MAC and DMA registers.

use crate::register::{RC, RW1C, RWNoModify};
use volatile_register::{RO, RW, WO};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/net/ethernet/spacemit/k1_emac.h
// https://github.com/spacemit-com/docs-chip/blob/d68a0caf7024a605f44ed818d41bab6786b6c999/en/key_stone/k1/k1_docs/k1_usermanual/15.High-Speed_Interface_System.md#1535-register-description
// PTP and AVB extensions are not part of this driver-backed register view.

/// K1/M1 Ethernet MAC and DMA registers.
#[repr(C)]
pub struct RegisterBlock {
    /// dma configuration.
    pub dma_configuration: RWNoModify<u32>,
    /// dma control.
    pub dma_control: RW<u32>,
    /// dma status irq.
    pub dma_status_irq: RW1C<u32>,
    /// dma interrupt enable.
    pub dma_interrupt_enable: RW<u32>,
    /// dma transmit auto poll counter.
    pub dma_transmit_auto_poll_counter: RW<u32>,
    /// dma transmit poll demand.
    pub dma_transmit_poll_demand: WO<u32>,
    /// dma receive poll demand.
    pub dma_receive_poll_demand: WO<u32>,
    /// dma transmit base address.
    pub dma_transmit_base_address: RW<u32>,
    /// dma receive base address.
    pub dma_receive_base_address: RW<u32>,
    /// dma missed frame counter.
    pub dma_missed_frame_counter: RC<u32>,
    /// dma stop flush counter.
    pub dma_stop_flush_counter: RW<u32>,
    /// dma receive irq mitigation ctrl.
    pub dma_receive_irq_mitigation_ctrl: RW<u32>,
    /// dma current transmit descriptor pointer.
    pub dma_current_transmit_descriptor_pointer: RO<u32>,
    /// dma current transmit buffer pointer.
    pub dma_current_transmit_buffer_pointer: RO<u32>,
    /// dma current receive descriptor pointer.
    pub dma_current_receive_descriptor_pointer: RO<u32>,
    /// dma current receive buffer pointer.
    pub dma_current_receive_buffer_pointer: RO<u32>,
    _padding_0x040: [u32; 48],
    /// mac global control.
    pub mac_global_control: RWNoModify<u32>,
    /// mac transmit control.
    pub mac_transmit_control: RW<u32>,
    /// mac receive control.
    pub mac_receive_control: RW<u32>,
    /// mac maximum frame size.
    pub mac_maximum_frame_size: RW<u32>,
    /// mac transmit jabber size.
    pub mac_transmit_jabber_size: RW<u32>,
    /// mac receive jabber size.
    pub mac_receive_jabber_size: RW<u32>,
    /// mac address control.
    pub mac_address_control: RW<u32>,
    /// mac mdio clk div.
    pub mac_mdio_clk_div: RW<u32>,
    /// mac address1 high.
    pub mac_address1_high: RW<u32>,
    /// mac address1 med.
    pub mac_address1_med: RW<u32>,
    /// mac address1 low.
    pub mac_address1_low: RW<u32>,
    /// mac address2 high.
    pub mac_address2_high: RW<u32>,
    /// mac address2 med.
    pub mac_address2_med: RW<u32>,
    /// mac address2 low.
    pub mac_address2_low: RW<u32>,
    /// mac address3 high.
    pub mac_address3_high: RW<u32>,
    /// mac address3 med.
    pub mac_address3_med: RW<u32>,
    /// mac address3 low.
    pub mac_address3_low: RW<u32>,
    /// mac address4 high.
    pub mac_address4_high: RW<u32>,
    /// mac address4 med.
    pub mac_address4_med: RW<u32>,
    /// mac address4 low.
    pub mac_address4_low: RW<u32>,
    /// mac multicast hash table1.
    pub mac_multicast_hash_table1: RW<u32>,
    /// mac multicast hash table2.
    pub mac_multicast_hash_table2: RW<u32>,
    /// mac multicast hash table3.
    pub mac_multicast_hash_table3: RW<u32>,
    /// mac multicast hash table4.
    pub mac_multicast_hash_table4: RW<u32>,
    /// mac fc control.
    pub mac_fc_control: RW<u32>,
    /// mac fc pause frame generate.
    pub mac_fc_pause_frame_generate: RWNoModify<u32>,
    /// mac fc source address high.
    pub mac_fc_source_address_high: RW<u32>,
    /// mac fc source address med.
    pub mac_fc_source_address_med: RW<u32>,
    /// mac fc source address low.
    pub mac_fc_source_address_low: RW<u32>,
    /// mac fc destination address high.
    pub mac_fc_destination_address_high: RW<u32>,
    /// mac fc destination address med.
    pub mac_fc_destination_address_med: RW<u32>,
    /// mac fc destination address low.
    pub mac_fc_destination_address_low: RW<u32>,
    /// mac fc pause time value.
    pub mac_fc_pause_time_value: RW<u32>,
    /// mac fc high pause time.
    pub mac_fc_high_pause_time: RW<u32>,
    /// mac fc low pause time.
    pub mac_fc_low_pause_time: RW<u32>,
    /// mac fc pause high threshold.
    pub mac_fc_pause_high_threshold: RW<u32>,
    /// mac fc pause low threshold.
    pub mac_fc_pause_low_threshold: RW<u32>,
    _padding_0x194: [u32; 3],
    /// mac mdio control.
    pub mac_mdio_control: RWNoModify<u32>,
    /// mac mdio data.
    pub mac_mdio_data: RW<u32>,
    /// mac rx statctr control.
    pub mac_rx_statctr_control: RWNoModify<u32>,
    /// mac rx statctr data high.
    pub mac_rx_statctr_data_high: RO<u32>,
    /// mac rx statctr data low.
    pub mac_rx_statctr_data_low: RO<u32>,
    /// mac tx statctr control.
    pub mac_tx_statctr_control: RWNoModify<u32>,
    /// mac tx statctr data high.
    pub mac_tx_statctr_data_high: RO<u32>,
    /// mac tx statctr data low.
    pub mac_tx_statctr_data_low: RO<u32>,
    /// mac transmit fifo almost full.
    pub mac_transmit_fifo_almost_full: RW<u32>,
    /// mac transmit packet start threshold.
    pub mac_transmit_packet_start_threshold: RW<u32>,
    /// mac receive packet start threshold.
    pub mac_receive_packet_start_threshold: RW<u32>,
    _padding_0x1cc: [u32; 5],
    /// mac status irq.
    pub mac_status_irq: RW1C<u32>,
    /// mac interrupt enable.
    pub mac_interrupt_enable: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, dma_configuration), 0x000);
        assert_eq!(offset_of!(RegisterBlock, dma_control), 0x004);
        assert_eq!(offset_of!(RegisterBlock, dma_status_irq), 0x008);
        assert_eq!(offset_of!(RegisterBlock, dma_interrupt_enable), 0x00c);
        assert_eq!(
            offset_of!(RegisterBlock, dma_transmit_auto_poll_counter),
            0x010
        );
        assert_eq!(offset_of!(RegisterBlock, dma_transmit_poll_demand), 0x014);
        assert_eq!(offset_of!(RegisterBlock, dma_receive_poll_demand), 0x018);
        assert_eq!(offset_of!(RegisterBlock, dma_transmit_base_address), 0x01c);
        assert_eq!(offset_of!(RegisterBlock, dma_receive_base_address), 0x020);
        assert_eq!(offset_of!(RegisterBlock, dma_missed_frame_counter), 0x024);
        assert_eq!(offset_of!(RegisterBlock, dma_stop_flush_counter), 0x028);
        assert_eq!(
            offset_of!(RegisterBlock, dma_receive_irq_mitigation_ctrl),
            0x02c
        );
        assert_eq!(
            offset_of!(RegisterBlock, dma_current_transmit_descriptor_pointer),
            0x030
        );
        assert_eq!(
            offset_of!(RegisterBlock, dma_current_transmit_buffer_pointer),
            0x034
        );
        assert_eq!(
            offset_of!(RegisterBlock, dma_current_receive_descriptor_pointer),
            0x038
        );
        assert_eq!(
            offset_of!(RegisterBlock, dma_current_receive_buffer_pointer),
            0x03c
        );
        assert_eq!(offset_of!(RegisterBlock, mac_global_control), 0x100);
        assert_eq!(offset_of!(RegisterBlock, mac_transmit_control), 0x104);
        assert_eq!(offset_of!(RegisterBlock, mac_receive_control), 0x108);
        assert_eq!(offset_of!(RegisterBlock, mac_maximum_frame_size), 0x10c);
        assert_eq!(offset_of!(RegisterBlock, mac_transmit_jabber_size), 0x110);
        assert_eq!(offset_of!(RegisterBlock, mac_receive_jabber_size), 0x114);
        assert_eq!(offset_of!(RegisterBlock, mac_address_control), 0x118);
        assert_eq!(offset_of!(RegisterBlock, mac_mdio_clk_div), 0x11c);
        assert_eq!(offset_of!(RegisterBlock, mac_address1_high), 0x120);
        assert_eq!(offset_of!(RegisterBlock, mac_address1_med), 0x124);
        assert_eq!(offset_of!(RegisterBlock, mac_address1_low), 0x128);
        assert_eq!(offset_of!(RegisterBlock, mac_address2_high), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, mac_address2_med), 0x130);
        assert_eq!(offset_of!(RegisterBlock, mac_address2_low), 0x134);
        assert_eq!(offset_of!(RegisterBlock, mac_address3_high), 0x138);
        assert_eq!(offset_of!(RegisterBlock, mac_address3_med), 0x13c);
        assert_eq!(offset_of!(RegisterBlock, mac_address3_low), 0x140);
        assert_eq!(offset_of!(RegisterBlock, mac_address4_high), 0x144);
        assert_eq!(offset_of!(RegisterBlock, mac_address4_med), 0x148);
        assert_eq!(offset_of!(RegisterBlock, mac_address4_low), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, mac_multicast_hash_table1), 0x150);
        assert_eq!(offset_of!(RegisterBlock, mac_multicast_hash_table2), 0x154);
        assert_eq!(offset_of!(RegisterBlock, mac_multicast_hash_table3), 0x158);
        assert_eq!(offset_of!(RegisterBlock, mac_multicast_hash_table4), 0x15c);
        assert_eq!(offset_of!(RegisterBlock, mac_fc_control), 0x160);
        assert_eq!(
            offset_of!(RegisterBlock, mac_fc_pause_frame_generate),
            0x164
        );
        assert_eq!(offset_of!(RegisterBlock, mac_fc_source_address_high), 0x168);
        assert_eq!(offset_of!(RegisterBlock, mac_fc_source_address_med), 0x16c);
        assert_eq!(offset_of!(RegisterBlock, mac_fc_source_address_low), 0x170);
        assert_eq!(
            offset_of!(RegisterBlock, mac_fc_destination_address_high),
            0x174
        );
        assert_eq!(
            offset_of!(RegisterBlock, mac_fc_destination_address_med),
            0x178
        );
        assert_eq!(
            offset_of!(RegisterBlock, mac_fc_destination_address_low),
            0x17c
        );
        assert_eq!(offset_of!(RegisterBlock, mac_fc_pause_time_value), 0x180);
        assert_eq!(offset_of!(RegisterBlock, mac_fc_high_pause_time), 0x184);
        assert_eq!(offset_of!(RegisterBlock, mac_fc_low_pause_time), 0x188);
        assert_eq!(
            offset_of!(RegisterBlock, mac_fc_pause_high_threshold),
            0x18c
        );
        assert_eq!(offset_of!(RegisterBlock, mac_fc_pause_low_threshold), 0x190);
        assert_eq!(offset_of!(RegisterBlock, mac_mdio_control), 0x1a0);
        assert_eq!(offset_of!(RegisterBlock, mac_mdio_data), 0x1a4);
        assert_eq!(offset_of!(RegisterBlock, mac_rx_statctr_control), 0x1a8);
        assert_eq!(offset_of!(RegisterBlock, mac_rx_statctr_data_high), 0x1ac);
        assert_eq!(offset_of!(RegisterBlock, mac_rx_statctr_data_low), 0x1b0);
        assert_eq!(offset_of!(RegisterBlock, mac_tx_statctr_control), 0x1b4);
        assert_eq!(offset_of!(RegisterBlock, mac_tx_statctr_data_high), 0x1b8);
        assert_eq!(offset_of!(RegisterBlock, mac_tx_statctr_data_low), 0x1bc);
        assert_eq!(
            offset_of!(RegisterBlock, mac_transmit_fifo_almost_full),
            0x1c0
        );
        assert_eq!(
            offset_of!(RegisterBlock, mac_transmit_packet_start_threshold),
            0x1c4
        );
        assert_eq!(
            offset_of!(RegisterBlock, mac_receive_packet_start_threshold),
            0x1c8
        );
        assert_eq!(offset_of!(RegisterBlock, mac_status_irq), 0x1e0);
        assert_eq!(offset_of!(RegisterBlock, mac_interrupt_enable), 0x1e4);
        assert_eq!(size_of::<RegisterBlock>(), 0x1e8);
    }
}
