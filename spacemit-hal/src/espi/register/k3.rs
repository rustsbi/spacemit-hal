//! K3 enhanced spi host registers.

use crate::register::{RC, RW1C, RWNoModify};
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/docs-chip/blob/main/en/key_stone/k3/k3_docs/k3_usermanual/14_connectivity/espi.md
// Host interface only; the RCPU eSPI slave has a separate layout.

/// K3 enhanced spi host registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Downstream header and transmit-start command.
    pub dn_txhdr_0: RWNoModify<u32>,
    /// DN TXHDR 1.
    pub dn_txhdr_1: RW<u32>,
    /// DN TXHDR 2.
    pub dn_txhdr_2: RW<u32>,
    /// Downstream FIFO data port; no read-modify-write.
    pub dn_txdata_port: RWNoModify<u32>,
    /// UP RXHDR 0.
    pub up_rxhdr_0: RO<u32>,
    /// UP RXHDR 1.
    pub up_rxhdr_1: RO<u32>,
    /// Reading pops the receive FIFO.
    pub up_rxdata_port: RC<u32>,
    _padding_0x01c: [u32; 4],
    /// MASTER CAP.
    pub master_cap: RO<u32>,
    /// GLOBAL CONTROL 0.
    pub global_control_0: RW<u32>,
    /// GLOBAL CONTROL 1.
    pub global_control_1: RWNoModify<u32>,
    /// PR BASE ADDR MEM 0.
    pub pr_base_addr_mem_0: RW<u32>,
    /// PR BASE ADDR MEM 1.
    pub pr_base_addr_mem_1: RW<u32>,
    _padding_0x040: [u32; 1],
    /// SLAVE0 STS SHADOW.
    pub slave0_sts_shadow: RWNoModify<u32>,
    _padding_0x048: [u32; 8],
    /// SLAVE0 CONFIG.
    pub slave0_config: RW<u32>,
    /// SLAVE0 INT EN.
    pub slave0_int_en: RW<u32>,
    /// SLAVE0 INT STS.
    pub slave0_int_sts: RWNoModify<u32>,
    /// SLAVE0 RX MSG HDR0.
    pub slave0_rx_msg_hdr0: RO<u32>,
    /// SLAVE0 RX MSG HDR1.
    pub slave0_rx_msg_hdr1: RO<u32>,
    /// Reading pops the receive FIFO.
    pub slave0_rxmsg_data_port: RC<u32>,
    _padding_0x080: [u32; 6],
    /// SLAVE0 RXVW STS.
    pub slave0_rxvw_sts: RW1C<u32>,
    /// Reading acknowledges virtual-wire events (SLAVE0_RXVW).
    pub slave0_rxvw: RC<u32>,
    /// SLAVE0 RXVW DATA.
    pub slave0_rxvw_data: RW<u32>,
    /// SLAVE0 RXVW INDEX.
    pub slave0_rxvw_index: RW<u32>,
    /// SLAVE0 VW CTL.
    pub slave0_vw_ctl: RW<u32>,
    /// SLAVE0 VW POLARITY.
    pub slave0_vw_polarity: RW<u32>,
    /// SLAVE0 M2S STS.
    pub slave0_m2s_sts: RW1C<u32>,
    /// SLAVE0 M2S MASK.
    pub slave0_m2s_mask: RW<u32>,
    /// SLAVE0 S2M MASK.
    pub slave0_s2m_mask: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(RegisterBlock, dn_txhdr_0), 0x0);
        assert_eq!(offset_of!(RegisterBlock, dn_txhdr_1), 0x4);
        assert_eq!(offset_of!(RegisterBlock, dn_txhdr_2), 0x8);
        assert_eq!(offset_of!(RegisterBlock, dn_txdata_port), 0xc);
        assert_eq!(offset_of!(RegisterBlock, up_rxhdr_0), 0x10);
        assert_eq!(offset_of!(RegisterBlock, up_rxhdr_1), 0x14);
        assert_eq!(offset_of!(RegisterBlock, up_rxdata_port), 0x18);
        assert_eq!(offset_of!(RegisterBlock, master_cap), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, global_control_0), 0x30);
        assert_eq!(offset_of!(RegisterBlock, global_control_1), 0x34);
        assert_eq!(offset_of!(RegisterBlock, pr_base_addr_mem_0), 0x38);
        assert_eq!(offset_of!(RegisterBlock, pr_base_addr_mem_1), 0x3c);
        assert_eq!(offset_of!(RegisterBlock, slave0_sts_shadow), 0x44);
        assert_eq!(offset_of!(RegisterBlock, slave0_config), 0x68);
        assert_eq!(offset_of!(RegisterBlock, slave0_int_en), 0x6c);
        assert_eq!(offset_of!(RegisterBlock, slave0_int_sts), 0x70);
        assert_eq!(offset_of!(RegisterBlock, slave0_rx_msg_hdr0), 0x74);
        assert_eq!(offset_of!(RegisterBlock, slave0_rx_msg_hdr1), 0x78);
        assert_eq!(offset_of!(RegisterBlock, slave0_rxmsg_data_port), 0x7c);
        assert_eq!(offset_of!(RegisterBlock, slave0_rxvw_sts), 0x98);
        assert_eq!(offset_of!(RegisterBlock, slave0_rxvw), 0x9c);
        assert_eq!(offset_of!(RegisterBlock, slave0_rxvw_data), 0xa0);
        assert_eq!(offset_of!(RegisterBlock, slave0_rxvw_index), 0xa4);
        assert_eq!(offset_of!(RegisterBlock, slave0_vw_ctl), 0xa8);
        assert_eq!(offset_of!(RegisterBlock, slave0_vw_polarity), 0xac);
        assert_eq!(offset_of!(RegisterBlock, slave0_m2s_sts), 0xb0);
        assert_eq!(offset_of!(RegisterBlock, slave0_m2s_mask), 0xb4);
        assert_eq!(offset_of!(RegisterBlock, slave0_s2m_mask), 0xb8);
        assert_eq!(size_of::<RegisterBlock>(), 0xbc);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
